use std::net::SocketAddr;
use axum::Router;
use axum::extract::{State as AxumState, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::routing::{get};
use serde_json::json;
use crate::config::WebServerConfig;
use crate::state::State;

async fn websocket(ws: WebSocketUpgrade, AxumState(state): AxumState<State>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: State) {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let all = state.read().iter().map(|(key, val)| val.to_json(Some(key))).collect::<Vec<_>>();

    if socket.send(Message::Text(json!(all).to_string())).await.is_err() {
        println!("Error sending message");
    }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            println!("Received message: {:?}", msg);
        }
    }
}

pub async fn init(config: &WebServerConfig, state: State) -> anyhow::Result<()> {
    let address = SocketAddr::new(config.host().parse()?, config.port());
    let listener = tokio::net::TcpListener::bind(address).await?;

    let app = Router::new()
        .route("/", get(websocket))
        .with_state(state.clone());

    axum::serve(listener, app).await?;

    Ok(())
}
