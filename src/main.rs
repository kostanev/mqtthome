mod devices;
mod config;
mod state;
mod web_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::init()?;

    let state = state::State::default();
    state.setup(&config.devices());

    let state_clone = state.clone();
    let web_server = web_server::init(&config.web_server(), state_clone);

    tokio::join!(web_server);

    Ok(())
}
