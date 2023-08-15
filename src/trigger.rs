use crate::config::model::Config;
use crate::raspi::gpio::GpioControl;
use anyhow::Result;
use log::{debug, error, info};
use pcap::{Active, Capture, Error};

/**
A control structure for managing triggers, configured with a reference to a `Config` instance and a `GpioControl` instance
*/
pub struct TriggerControl<'c> {
    pub config: &'c Config,
    gpio: GpioControl<'c>,
}

impl<'c> TriggerControl<'c> {
    /// Constructs a new `TriggerControl` instance with the given `Config`.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` object used for configuration.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the constructed `TriggerControl` instance on success, or an `Err` variant if the construction fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::new();
    /// let trigger_control = TriggerControl::new(&config).unwrap();
    /// ```
    pub fn new(config: &'c Config) -> Result<Self> {
        let gpio = GpioControl::new(config)?;
        Ok(TriggerControl { config, gpio })
    }

    /// Runs the packet capturing process and starts a PWM on a GPIO pin based on certain conditions.
    ///
    /// # Arguments
    ///
    /// * `cancel` - A boolean flag indicating whether to cancel the capturing process.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut my_capturer = Capturer::new();
    /// my_capturer.run(&mut false);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if any issues occur during packet capturing or PWM starting.

    pub fn run(&mut self, cancel: &bool) -> Result<()> {
        debug!("Start capturing packets.");

        let mut capture = self.capture().unwrap();

        match capture.filter(self.config.get_filter(), true) {
            Ok(_) => debug!("Filter set sucesfully"),
            Err(e) => error!("Filter did not set due: {:?}", e),
        }

        while !*cancel {
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

    /// Creates a `Capture<Active>` object for packet capturing based on the device configuration.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant if the capture device fails to open.
    fn capture(&self) -> Result<Capture<Active>, Error> {
        let cap = Capture::from_device(self.config.get_device().clone());

        let active: Result<Capture<Active>, Error>;

        match cap {
            Ok(capture) => active = capture.immediate_mode(true).promisc(true).open(),
            Err(e) => active = Err(e),
        }

        active
    }
    /// Checks the received bytes against the trigger bytes configured in the `Capturer` instance.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes representing the received data to be checked.
    ///
    /// # Returns
    ///
    /// Returns a boolean value indicating whether the received bytes pass the trigger conditions.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut capturer = Capturer::new();
    /// let data = [0x12, 0x34, 0x56];
    /// assert_eq!(capturer.check_bytes(&data), false);
    /// ```
    fn check_bytes(&mut self, data: &[u8]) -> bool {
        info!("received data: {:?}", data);
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
    /// Retrieves the byte at the specified position from the given data slice.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes containing the data.
    /// * `pos` - A reference to the position of the byte to retrieve.
    ///
    /// # Returns
    ///
    /// Returns the u8 value representing the byte at the specified position.
    ///
    /// # Examples
    ///
    /// ```
    /// let capturer = Capturer::new();
    /// let data = [0x12, 0x34, 0x56];
    /// let byte = capturer.get_byte_at_position(&data, &1);
    /// assert_eq!(byte, 0x34);
    /// ```
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
