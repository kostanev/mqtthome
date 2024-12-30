use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::config::{DeviceConfig, DeviceType};
use crate::devices::{DeviceWrapper, ShellyDimmer2};

type Devices = HashMap<String, DeviceWrapper>;

#[derive(Clone, Debug, Default)]
pub struct State {
    devices: Arc<RwLock<Devices>>
}

impl State {
    pub fn setup(&self, device_configurations: &Vec<DeviceConfig>) {
        for device in device_configurations {
            let device_id = device.id().to_string();
            let device_instance = match device.device_type() {
                DeviceType::ShellyDimmer2 => DeviceWrapper::ShellyDimmer2(ShellyDimmer2::default())
            };

            self.devices.write().unwrap().insert(device_id, device_instance);
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, Devices> {
        self.devices.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, Devices> {
        self.devices.write().unwrap()
    }
}
