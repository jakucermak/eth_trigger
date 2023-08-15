use std::string::ToString;
use std::time::Duration;

use log::{error, info};
use pcap::Device;
use serde::Deserialize;

/// Represents a configuration for a device.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    init_run: bool,                    // Whether to run initialization
    interface: String,                 // The name of the network interface
    filter: String,                    // The filter string for capturing packets
    pwm_delay_ms: u64,                 // The PWM delay in milliseconds
    frame_cnt: u16,                    // The number of frames
    gpio_pin: u16,                     // The GPIO pin number
    trig_bytes: Option<Vec<TrigByte>>, // Optional trigger bytes
}

/// Represents a trigger byte.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct TrigByte {
    pos: i32,  // The position of the trigger byte
    value: u8, // The value of the trigger byte
}

impl Default for Config {
    /// Creates a new instance of `Config` with default values.
    fn default() -> Self {
        Config {
            init_run: true,
            interface: "eth0".to_string(),
            filter: "".to_string(),
            pwm_delay_ms: 100,
            frame_cnt: 1,
            gpio_pin: 1,
            trig_bytes: Some(vec![]),
        }
    }
}

impl Config {
    /// Returns the device with the specified interface name.
    /// If the interface name is not found, it falls back to the first available interface.
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

    /// Returns the filter string.
    pub fn get_filter(&self) -> &str {
        &self.filter
    }

    /// Returns the number of frames.
    pub fn get_frame_count(&self) -> &u16 {
        &self.frame_cnt
    }

    /// Returns the PWM delay as a `Duration` value.
    pub fn get_delay(&self) -> Duration {
        Duration::from_millis(self.pwm_delay_ms)
    }

    /// Returns the GPIO pin number.
    pub fn get_gpio_pin(&self) -> &u16 {
        &self.gpio_pin
    }

    /// Returns the trigger bytes.
    pub fn get_trigger_bytes(&self) -> &Option<Vec<TrigByte>> {
        &self.trig_bytes
    }

    /// Returns whether the initialization run should be performed.
    pub fn get_init_run(&self) -> &bool {
        &self.init_run
    }
}

impl TrigByte {
    /// Returns the position of the trigger byte.
    pub fn get_pos(&self) -> &i32 {
        &self.pos
    }

    /// Returns the value of the trigger byte.
    pub fn get_val(&self) -> &u8 {
        &self.value
    }
}
