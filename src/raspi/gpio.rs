use crate::config::model::Config;
use anyhow::Result;
use gpio::{sysfs::SysFsGpioOutput, GpioOut};
use log::{debug, error};
use std::thread::sleep;

pub struct GpioControl<'c> {
    pub config: &'c Config,
    gpio: SysFsGpioOutput,
}

impl<'c> GpioControl<'c> {
    pub fn new(config: &'c Config) -> Result<Self> {
        let gpio_18 = gpio::sysfs::SysFsGpioOutput::open(*config.get_gpio_pin())
            .expect("Unable to open GPIO12");

        Ok(GpioControl {
            config,
            gpio: gpio_18,
        })
    }

    pub fn start_pwm(&mut self) -> Result<()> {
        let mut value = false;
        for _ in 0..*self.config.get_frame_count() {
            self.gpio
                .set_value(value)
                .expect("Unable to set value on GPIO12");
            self.toggle_value(&mut value);
            sleep(self.config.get_delay());
        }

        match self.gpio.set_value(false) {
            Ok(_) => debug!("Value set sucessfully"),
            Err(e) => error!("Value did not set: {:?}", e),
        }
        Ok(())
    }

    fn toggle_value(&self, value: &mut bool) {
        *value = !*value;
    }
}
