FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    clang \
    cmake \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Install specific Rust components for Solana
RUN rustup toolchain install stable
RUN rustup component add rustfmt
RUN rustup target add sbf-solana-solana

# Install Solana CLI tools
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# Install cargo-sbf
RUN cargo install --git https://github.com/solana-labs/solana --tag v1.18.0 solana-program

WORKDIR /app

# Verify installation
RUN solana --version
RUN cargo --version
RUN rustc --version
