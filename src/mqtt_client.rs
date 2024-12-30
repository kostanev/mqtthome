use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::time::Duration;
use crate::config::MqttClientConfig;

pub struct MqttClient {
    client: AsyncClient,
    event_loop: EventLoop
}

impl MqttClient {
    pub async fn init(config: &MqttClientConfig) -> anyhow::Result<Self> {
        let mut options = MqttOptions::new(config.id(), config.host(), config.port());
        options.set_credentials(config.user(), config.pass());
        options.set_keep_alive(Duration::from_secs(5));

        let (client, event_loop) = AsyncClient::new(options, 10);
        client.subscribe("#", QoS::AtMostOnce).await.unwrap();

        Ok(Self { client, event_loop })
    }

    pub async fn send(&self, topic: &str, payload: &str) {
        self.client.publish(topic, QoS::ExactlyOnce, false, payload.as_bytes()).await.unwrap();
    }

    pub async fn receive(&mut self) -> Option<(String, String)> {
        let event = self.event_loop.poll().await.unwrap();

        let Event::Incoming(data) = event else {
            return None;
        };

        let Incoming::Publish(message) = data else {
            return None;
        };

        let topic = message.topic;
        let payload = String::from_utf8(message.payload.to_vec()).unwrap();

        Some((topic, payload))
    }
}
