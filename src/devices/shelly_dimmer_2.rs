use serde_json::Value;
use serde::Serialize;
use crate::devices::device::Device;

const DEVICE_TYPE: &str = file!();

#[derive(Debug, Default, Serialize)]
pub struct ShellyDimmer2 {
    on: bool,
    brightness: u8,
    temperature: f32,
    over_power: bool,
    over_temperature: bool
}

impl ShellyDimmer2 {
    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn get_brightness(&self) -> u8 {
        self.brightness
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn is_over_power(&self) -> bool {
        self.over_power
    }

    pub fn is_over_temperature(&self) -> bool {
        self.over_temperature
    }
}

impl Device for ShellyDimmer2 {
    fn get_type(&self) -> &'static str {
        DEVICE_TYPE
    }

    fn parse(&mut self, command: &str, value: &str) {
        match command {
            "light/0/status" => {
                let status: Value = serde_json::from_str(value).map_err(|e| e.to_string()).unwrap();

                self.on = status
                    .get("ison").ok_or(format!("Cannot parse 'ison' from {}", status)).unwrap()
                    .as_bool().ok_or(format!("Cannot covert 'ison' to bool from {}", status)).unwrap();

                self.brightness = status
                    .get("brightness").ok_or(format!("Cannot parse 'brightness' from {}", status)).unwrap()
                    .as_u64().ok_or(format!("Cannot covert 'brightness' to u8 from {}", status)).unwrap() as u8;
            },
            "temperature" => {
                self.temperature = value.parse().map_err(|_| "Cannot parse 'temperature'").unwrap();
            },
            "overpower" => {
                self.over_power = value == "1";
            },
            "overtemperature" => {
                self.over_temperature = value == "1";
            },
            _ => ()
        }
    }
}