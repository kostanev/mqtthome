use serde::Serialize;
use serde_json::{json, Map, Value};
use crate::devices::Device;
use crate::state::CommandTransmitter;

pub struct ShellyDimmer2Command {
    pub(crate) set_on: Option<bool>,
    pub(crate) set_brightness: Option<u8>
}

#[derive(Debug, Default, Serialize)]
pub struct State {
    on: bool,
    brightness: u8,
    temperature: f32,
    over_power: bool,
    over_temperature: bool
}

#[derive(Debug, Serialize)]
pub struct ShellyDimmer2 {
    id: String,
    name: String,
    state: State,
    #[serde(skip_serializing)]
    command_tx: CommandTransmitter,
}

impl ShellyDimmer2 {
    pub async fn run_command(&self, cmd: ShellyDimmer2Command) {
        let mut data = Map::new();

        if let Some(on) = cmd.set_on {
            data.insert(String::from("turn"), Value::from(if on { "on" } else { "off" }));
        }

        if let Some(brightness) = cmd.set_brightness {
            data.insert(String::from("brightness"), Value::from(brightness));
        }

        if data.len() > 0 {
            let key = format!("{}/light/0/set", self.id);
            self.command_tx.send((key, json!(data).to_string())).await.ok();
        }
    }
}

impl Device for ShellyDimmer2 {
    fn init(id: String, name: String, command_tx: CommandTransmitter) -> Self {
        Self { id, name, command_tx, state: Default::default() }
    }

    fn parse(&mut self, command: &str, value: &str) {
        match command {
            "light/0/status" => {
                let status: Value = serde_json::from_str(value).map_err(|e| e.to_string()).unwrap();

                self.state.on = status
                    .get("ison").ok_or(format!("Cannot parse 'ison' from {}", status)).unwrap()
                    .as_bool().ok_or(format!("Cannot covert 'ison' to bool from {}", status)).unwrap();

                self.state.brightness = status
                    .get("brightness").ok_or(format!("Cannot parse 'brightness' from {}", status)).unwrap()
                    .as_u64().ok_or(format!("Cannot covert 'brightness' to u8 from {}", status)).unwrap() as u8;
            },
            "temperature" => {
                self.state.temperature = value.parse().map_err(|_| "Cannot parse 'temperature'").unwrap();
            },
            "overpower" => {
                self.state.over_power = value == "1";
            },
            "overtemperature" => {
                self.state.over_temperature = value == "1";
            },
            _ => ()
        }
    }
}
