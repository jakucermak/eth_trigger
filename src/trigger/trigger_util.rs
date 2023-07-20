use std::process;

use pcap::Device;

#[derive(Debug)]
enum Status {
    Triggering,
    Idle,
}
#[derive(Debug)]
pub struct TriggerConfig {
    status: Status,
    interface: Device,
    cap_filename: String,
    filter: String,
}

impl TriggerConfig {
    pub fn configure(interface: &str, filename: &str, filter: &str) -> TriggerConfig {
        let device = match Device::list() {
            Ok(devices) => devices.into_iter().find(|device| device.name == interface),
            Err(e) => {
                println!("Exiting due to: {}", e);
                process::exit(1);
            }
        };
        match device {
            Some(device) => TriggerConfig {
                status: Status::Idle,
                interface: device.to_owned(),
                cap_filename: filename.to_string(),
                filter: filter.to_string(),
            },
            None => {
                println!("Exiting, no device found. Check configuration");
                process::exit(1);
            }
        }
    }

    pub fn get_device(&self) -> &Device {
        &self.interface
    }

    pub fn get_filter(&self) -> &str {
        &self.filter
    }
}
