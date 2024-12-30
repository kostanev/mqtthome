use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    web_server: WebServerConfig,
    mqtt_client: MqttClientConfig,
    devices: Vec<DeviceConfig>
}

impl Config {
    pub fn init() -> anyhow::Result<Self> {
        let file = File::open("config.yaml")?;
        let config: Self = serde_yml::from_reader(file)?;

        Ok(config)
    }

    pub fn web_server(&self) -> &WebServerConfig {
        &self.web_server
    }

    pub fn mqtt_client(&self) -> &MqttClientConfig {
        &self.mqtt_client
    }

    pub fn devices(&self) -> &Vec<DeviceConfig> {
        &self.devices
    }
}

#[derive(Debug, Deserialize)]
pub struct WebServerConfig {
    host: String,
    port: u16
}

impl WebServerConfig {
    pub fn host(&self) -> &str { &self.host }
    pub fn port(&self) -> u16 { self.port }
}

#[derive(Debug, Deserialize)]
pub struct MqttClientConfig {
    id: String,
    host: String,
    port: u16,
    user: String,
    pass: String
}

impl MqttClientConfig {
    pub fn id(&self) -> &str { &self.id }
    pub fn host(&self) -> &str { &self.host }
    pub fn port(&self) -> u16 { self.port }
    pub fn user(&self) -> &str { &self.user }
    pub fn pass(&self) -> &str { &self.pass }
}

#[derive(Debug, Deserialize)]
pub enum DeviceType {
    ShellyDimmer2
}

#[derive(Debug, Deserialize)]
pub struct DeviceConfig {
    id: String,
    name: String,
    #[serde(rename = "type")]
    device_type: DeviceType
}

impl DeviceConfig {
    pub fn id(&self) -> &str { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn device_type(&self) -> &DeviceType { &self.device_type }
}
