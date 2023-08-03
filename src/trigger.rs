mod trigger_util;

use crate::config::model::Config;
use crate::raspi::gpio::GpioControl;
use anyhow::Result;
use log::debug;
use pcap::{Active, Capture, Error, Packet};
use std::process;

pub struct TriggerControl<'c> {
    pub config: &'c Config,
    gpio: GpioControl<'c>,
}

impl<'c> TriggerControl<'c> {
    pub fn new(config: &'c Config) -> Result<Self> {
        let gpio = GpioControl::new(config)?;
        Ok(TriggerControl { config, gpio })
    }

    pub fn run(&mut self) -> Result<()> {
        debug!("Start capturing packets.");
        let mut packet;

        let mut capture = self.capture().unwrap();

        capture.filter(self.config.get_filter(), true);

        packet = capture.next_packet();

        match packet {
            Ok(packet) => {
                if self.check_bytes(packet) {
                    self.gpio.start_pwm();
                }
            }
            Err(e) => (),
        }
        Ok(())
    }

    fn capture(&self) -> Result<Capture<Active>, Error> {
        let cap = Capture::from_device(self.config.get_device().clone());

        let active: Result<Capture<Active>, Error>;

        match cap {
            Ok(capture) => active = capture.immediate_mode(true).promisc(true).open(),
            Err(e) => active = Err(e),
        }

        active
    }

    fn check_bytes(&self, packet: Packet) -> bool {
        println!("{:?}", packet.data);
        if false {
            return true;
        }
        false
    }
}
