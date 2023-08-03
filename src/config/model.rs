use std::time::Duration;

use log::{error, info};
use pcap::Device;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    interface: String,
    filename: String,
    filter: String,
    pwm_delay_ms: u64,
    frame_cnt: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interface: "eth0".to_string(),
            filename: "capture.pcap".to_string(),
            filter: "".to_string(),
            pwm_delay_ms: 100,
            frame_cnt: 1,
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
}
