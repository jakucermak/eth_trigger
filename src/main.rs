#![feature(ascii_char)]

use trigger::Trigger;

mod config;
mod trigger;

fn main() {
    let trigger = Trigger::configure(&config::get_configuration());

    trigger.run()
}
