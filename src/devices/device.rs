use enum_dispatch::enum_dispatch;
use serde::Serialize;
use serde_json::{json, Value};
use crate::devices::ShellyDimmer2;

#[enum_dispatch]
#[derive(Debug, Serialize)]
pub enum DeviceWrapper {
    ShellyDimmer2
}

impl DeviceWrapper {
    pub fn to_json(&self, id: Option<&str>) -> Value {
        let res = match self {
            DeviceWrapper::ShellyDimmer2(inner) => inner
        };
        let mut json = json!(res);
        if let Some(id) = id {
            json["id"] = id.into();
        }
        json
    }
}

#[enum_dispatch(DeviceWrapper)]
pub trait Device {
    fn get_type(&self) -> &str;
    fn parse(&mut self, command: &str, value: &str);
}
