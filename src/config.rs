pub mod model;
use std::fs;

use directories::ProjectDirs;
use model::Config;

pub fn get_configuration() -> Config {

    let proj_dirs = ProjectDirs::from("dev", "jakub", "eth_trigger").unwrap();

    let config_file = fs::read_to_string(proj_dirs.config_dir().join("config.toml"));

    println!("Getting config from: {:?}", proj_dirs.config_dir());

    match config_file {
        Ok(file) => match toml::from_str(&file) {
            Ok(config) => {
                println!("Configuration: {:?}", config);
                config
            }
            Err(e) => {
                println!("{:?}", e);
                Config::default()
            }
        },
        Err(e) => {
            println!("{:?}", e);
            println!("Loading default config");
            Config::default()
        }
    }
}
