use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Program {
    pub program_id: String,
    pub mpl_core_program_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Solana {
    pub keypair_file_directory: String,
    pub rpc_client_url: String
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub program: Program,
    pub solana: Solana,
}

impl AppConfig {
    pub fn new(config_file: impl AsRef<str>) -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(config_file.as_ref()))
            .build()?;

        config.try_deserialize()
    }
}
