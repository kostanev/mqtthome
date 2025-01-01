use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::config::{DeviceConfig, DeviceType};
use crate::devices::{Device, DeviceWrapper, ShellyDimmer2};

pub type CommandTransmitter = mpsc::Sender<(String, String)>;
pub type CommandReceiver = mpsc::Receiver<(String, String)>;

type Devices = HashMap<String, DeviceWrapper>;

#[derive(Clone, Debug, Default)]
pub struct State {
    devices: Arc<RwLock<Devices>>
}

impl State {
    pub async fn setup_devices(&self, device_configurations: &Vec<DeviceConfig>) -> CommandReceiver {
        let (tx, mut rx) = mpsc::channel(32);

        for device in device_configurations {
            let tx_clone = tx.clone();

            let device_id = device.id().to_string();
            let device_name = device.name().to_string();
            let device_instance = match device.device_type() {
                DeviceType::ShellyDimmer2 => {
                    DeviceWrapper::ShellyDimmer2(ShellyDimmer2::init(device_id.clone(), device_name, tx_clone))
                }
            };

            self.devices.write().await.insert(device_id, device_instance);
        }

        rx
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, Devices> {
        self.devices.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, Devices> {
        self.devices.write().await
    }
}
