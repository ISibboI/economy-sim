use log::info;
use simplelog::TermLogger;

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
