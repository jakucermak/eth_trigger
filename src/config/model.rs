use std::time::Duration;

use log::{error, info};
use pcap::Device;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    init_run: bool,
    interface: String,
    filter: String,
    pwm_delay_ms: u64,
    frame_cnt: u16,
    gpio_pin: u16,
    trig_bytes: Option<Vec<TrigByte>>,
}
#[derive(Deserialize, Debug)]
pub struct TrigByte {
    pos: i32,
    value: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interface: "eth0".to_string(),
            filter: "".to_string(),
            pwm_delay_ms: 100,
            frame_cnt: 1,
            gpio_pin: 1,
            trig_bytes: Some(vec![]),
            init_run: true,
        }
    }
}

impl Config {
    pub fn get_device(&self) -> Device {
        let device: Device = match Device::list() {
            Ok(devices) => devices
                .into_iter()
                .find(|device| device.name == self.interface)
                .unwrap(),
            Err(e) => {
                error!("{}", e);
                info!("using first available interface");
                Device::lookup().unwrap().unwrap()
            }
        };

        device
    }

    pub fn get_filter(&self) -> &str {
        &self.filter
    }

    pub fn get_frame_count(&self) -> &u16 {
        &self.frame_cnt
    }

    pub fn get_delay(&self) -> Duration {
        Duration::from_millis(self.pwm_delay_ms)
    }

    pub fn get_gpio_pin(&self) -> &u16 {
        &self.gpio_pin
    }

    pub fn get_trigger_bytes(&self) -> &Option<Vec<TrigByte>> {
        &self.trig_bytes
    }

    pub fn get_init_run(&self) -> &bool {
        &self.init_run
    }
}

impl TrigByte {
    pub fn get_pos(&self) -> &i32 {
        &self.pos
    }

    pub fn get_val(&self) -> &u8 {
        &self.value
    }
}
