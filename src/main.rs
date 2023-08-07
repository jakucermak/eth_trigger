use config::get_configuration;
use trigger::TriggerControl;

mod config;
mod raspi;
mod trigger;

fn main() {
    let config = get_configuration();

    let mut trigger = TriggerControl::new(&config).unwrap();

    match trigger.run() {
        Ok(_) => {
            println!("Done sucesfully");
        }
        Err(e) => {
            println!("Finished with error: {}", e);
        }
    }
}
