use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use tokio::sync::mpsc;
use crate::config::MqttClientConfig;

pub type MqttClientTxRx = (
    mpsc::UnboundedSender<(String, String)>,
    mpsc::UnboundedReceiver<(String, String)>
);

pub fn init(config: &MqttClientConfig) -> anyhow::Result<MqttClientTxRx> {
    let (tx_in, mut rx_in) = mpsc::unbounded_channel();
    let (tx_out, rx_out) = mpsc::unbounded_channel();

    let mut options = MqttOptions::new(config.id(), config.host(), config.port());
    options.set_credentials(config.user(), config.pass());
    options.set_keep_alive(Duration::from_secs(5));

    let (client, mut event_loop) = AsyncClient::new(options.clone(), 10);

    tokio::spawn(async move {
        client.subscribe("#", QoS::ExactlyOnce).await.unwrap();

        while let Some((topic, payload)) = rx_in.recv().await {
            client.publish(topic, QoS::ExactlyOnce, false, payload).await.unwrap();
        }
    });

    tokio::spawn(async move {
        while let Ok(event) = event_loop.poll().await {
            let Event::Incoming(data) = event else {
                continue;
            };

            let Incoming::Publish(message) = data else {
                continue;
            };

            let topic = message.topic;
            let payload = String::from_utf8(message.payload.to_vec()).unwrap();

            tx_out.send((topic, payload)).unwrap();
        }
    });

    Ok((tx_in, rx_out))
}
