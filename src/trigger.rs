use crate::config::model::Config;
use crate::raspi::gpio::GpioControl;
use anyhow::Result;
use log::{debug, error};
use pcap::{Active, Capture, Error};

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

        let mut capture = self.capture().unwrap();

        match capture.filter(self.config.get_filter(), true) {
            Ok(_) => debug!("Filter set sucesfully"),
            Err(e) => error!("Filter did not set due: {:?}", e),
        }

        loop {
            let packet = capture.next_packet();
            match packet {
                Ok(p) => {
                    if self.check_bytes(&p.data) {
                        match self.gpio.start_pwm() {
                            Ok(_) => debug!("GPIO stared sucesfully"),
                            Err(e) => error!("GPIO did not start sucesfully due to: {:?}", e),
                        }
                    }
                }
                Err(e) => error!("{:?}", e),
            }
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

    fn check_bytes(&mut self, data: &[u8]) -> bool {
        println!("received data: {:?}", data);
        let mut is_valid: bool = *self.config.get_init_run();
        if let Some(trig_bytes) = self.config.get_trigger_bytes() {
            for byte in trig_bytes {
                if self.get_byte_at_position(&data, byte.get_pos()) == *byte.get_val() {
                    is_valid = true;
                } else {
                    is_valid = false;
                    break;
                }
            }
        }

        is_valid
    }

    fn get_byte_at_position(&self, data: &[u8], pos: &i32) -> u8 {
        let byte: &u8;
        if pos < &0 {
            byte = &data[data.len() - (*pos * -1) as usize];
        } else {
            byte = &data[*pos as usize];
        }

        *byte
    }
}
