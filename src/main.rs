mod devices;
mod config;
mod mqtt_client;
mod state;
mod web_server;

// use std::sync::Arc;
// use rumqttc::QoS;
// use tokio::sync::Mutex;
use crate::{
    devices::Device,
    config::Config,
    // state::State
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init()?;

    // let state = State::default();
    // let mut commands_rx = state.setup_devices(&config.devices()).await;

    // let state_clone = state.clone();
    let (tx_ws, mut rx_ws) = web_server::init(&config.web_server()).await?;
    let (tx_mq, mut rx_mq) = mqtt_client::init(&config.mqtt_client())?;

    let x = tokio::spawn(async move {
        while let Ok(msg) = rx_ws.recv().await {
            let x = msg.split(',').collect::<Vec<&str>>();

            tx_mq.send((
                x.get(0).unwrap_or(&"none").to_string(),
                x.get(1).unwrap_or(&"none").to_string()
            )).unwrap();
        }
    });

    let y = tokio::spawn(async move {
        while let Some((topic, payload)) = rx_mq.recv().await {
            let msg = format!("{topic} : {payload}");
            // println!("MQTT: {}", msg);
            tx_ws.send(msg).ok();
        }
    });

    tokio::join!(x, y);

    Ok(())
}
