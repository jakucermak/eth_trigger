use config::{get_configuration, model::Config};
use trigger::TriggerControl;
mod config;
mod raspi;
mod trigger;
use log::{error, info};

fn main() {
    env_logger::init();

    let config: &Config = &get_configuration();

    let mut trigger = TriggerControl::new(config).unwrap();

    match trigger.run(&false) {
        Ok(_) => {
            info!("Done sucesfully");
        }

        Err(e) => {
            error!("Finished with error: {}", e);
        }
    }
}
