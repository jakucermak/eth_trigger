mod trigger_util;

use crate::config::model::Config;
use pcap::{Active, Capture};
use trigger_util::TriggerConfig;

pub struct Trigger {
    config: TriggerConfig,
}

impl Trigger {
    pub fn run(self) {
        println!("Start capturing packets.");
        let mut cap = pcap::Capture::from_device(self.config.get_device().clone())
            .unwrap()
            .immediate_mode(true)
            .promisc(true)
            .open()
            .unwrap();
        cap.filter(self.config.get_filter().clone(), true).unwrap();
        self.capture(cap);
    }

    fn capture(self, mut cap: Capture<Active>) {
        match cap.next_packet() {
            Ok(packet) => {
                println!("data: {:?}", packet.data);
                // println!("as_bytes: {:?}", packet.as_bytes())
            }
            Err(e) => println!("{:?}", e),
        };
    }
    pub fn configure(config: &Config) -> Trigger {
        Trigger {
            config: TriggerConfig::configure(&config.interface, &config.filename, &config.filter),
        }
    }
}
