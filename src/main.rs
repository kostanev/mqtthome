mod devices;
mod config;
mod state;

fn main() -> anyhow::Result<()> {
    let config = config::Config::init()?;

    let state = state::State::default();
    state.setup(&config.devices());

    println!("{:#?}", &state);

    Ok(())
}
