use std::net::SocketAddr;
use axum::Router;
use axum::extract::{State as AxumState, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::routing::{get};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use crate::config::WebServerConfig;

pub type WebServerTxRx = (
    broadcast::Sender<String>,
    broadcast::Receiver<String>
);

#[derive(Clone)]
struct WebServerState {
    tx_out: broadcast::Sender<String>,
    tx_in: broadcast::Sender<String>
}

pub async fn init(config: &WebServerConfig) -> anyhow::Result<WebServerTxRx> {
    let (tx_in, _) = broadcast::channel(32);
    let (tx_out, rx_out) = broadcast::channel(32);

    let address = SocketAddr::new(config.host().parse()?, config.port());

    let tx_in_clone = tx_in.clone();

    tokio::spawn(async move {
        let listener = TcpListener::bind(address).await.unwrap();
        let app = Router::new()
            .route("/", get(websocket))
            .with_state(WebServerState { tx_out, tx_in: tx_in_clone });

        axum::serve(listener, app).await.unwrap();
    });

    Ok((tx_in, rx_out))
}

async fn websocket(ws: WebSocketUpgrade, AxumState(state): AxumState<WebServerState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: WebServerState) {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let (sender, receiver) = socket.split();

    tokio::spawn(write(sender, state.tx_in.subscribe()));
    tokio::spawn(read(receiver, state.tx_out));
}

async fn write(mut sender: SplitSink<WebSocket, Message>, mut rx: broadcast::Receiver<String>) {
    while let Ok(msg) = rx.recv().await {
        sender.send(Message::Text(msg.into())).await.unwrap();
    }
}

async fn read(mut receiver: SplitStream<WebSocket>, tx: broadcast::Sender<String>) {
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            tx.send(String::from(msg.to_text().unwrap())).unwrap();
        }
    }
}

