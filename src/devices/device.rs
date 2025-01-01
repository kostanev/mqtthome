use serde::Serialize;
use crate::state::CommandTransmitter;
use crate::devices::ShellyDimmer2;

pub trait Device {
    fn init(id: String, name: String, command_tx: CommandTransmitter) -> Self;
    fn parse(&mut self, command: &str, value: &str);
}

#[derive(Debug, Serialize)]
pub enum DeviceWrapper {
    ShellyDimmer2(ShellyDimmer2)
}

impl DeviceWrapper {
    pub fn parse(&mut self, command: &str, value: &str) {
        match self {
            DeviceWrapper::ShellyDimmer2(d) => d.parse(command, value)
        };
    }
}
