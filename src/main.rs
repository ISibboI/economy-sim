use log::info;
use simplelog::TermLogger;

mod recipes;
mod time;
mod wares;

fn main() {
    TermLogger::init(
        log::LevelFilter::Info,
        Default::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    info!("Hello, world!");
}
