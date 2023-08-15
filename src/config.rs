pub mod model;

use std::fs;

use directories::ProjectDirs;
use log::{error, info, warn};
use model::Config;

pub fn get_configuration<'a>() -> Config {
    let proj_dirs = ProjectDirs::from("dev", "jakub", "eth_trigger").unwrap();

    let config_file = fs::read_to_string(proj_dirs.config_dir().join("config.toml"));

    info!("Getting config from: {:?}", proj_dirs.config_dir());

    match config_file {
        Ok(file) => {
            let config: Result<Config, toml::de::Error> = toml::from_str(&file);
            match config {
                Ok(config) => {
                    info!("Configuration: {:?}", config);
                    config
                }
                Err(e) => {
                    error!("{:?}", e.to_string());
                    Config::default()
                }
            }
        }
        Err(e) => {
            warn!("{:?} \n Loading default config", e.to_string());
            Config::default()
        }
    }
}
