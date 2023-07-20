use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub interface: String,
    pub filename: String,
    pub filter: String,
    pub pwm_delay_ms: u16,
    pub frame_cnt: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interface: "eth0".to_string(),
            filename: "capture.pcap".to_string(),
            filter: "".to_string(),
            pwm_delay_ms: 0,
            frame_cnt: 1,
        }
    }
}
