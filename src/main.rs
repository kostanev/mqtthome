mod devices;
mod config;
mod mqtt_client;
mod state;
mod web_server;

use crate::{
    devices::Device,
    config::Config,
    mqtt_client::MqttClient,
    state::State
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init()?;

    let state = State::default();
    state.setup(&config.devices());

    let state_clone = state.clone();
    let web_server_task = web_server::init(&config.web_server(), state_clone);

    let mut mqtt_client = MqttClient::init(&config.mqtt_client()).await?;

    let state_clone = state.clone();
    let mqtt_processing_task = tokio::spawn(async move {
        loop {
            if let Some((topic, payload)) = mqtt_client.receive().await {
                state_clone.write().iter_mut().for_each(|(key, device)| {
                    if topic.contains(key) {
                        let command = topic.chars().skip(key.chars().count() + 1).collect::<String>();
                        device.parse(&command, &payload);
                    }
                });
            }
        }
    });

    tokio::join!(web_server_task, mqtt_processing_task);

    Ok(())
}
