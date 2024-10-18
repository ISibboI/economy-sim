use factory::Factory;
use log::info;
use money::Money;
use recipe::{ProductionRate, Recipe};
use simplelog::TermLogger;
use ware::{Ware, WareAmount};
use world::World;

mod factory;
mod market;
mod money;
mod recipe;
mod time;
mod ware;
mod warehouse;
mod world;

fn main() {
    TermLogger::init(
        log::LevelFilter::Info,
        Default::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    info!("Creating world");
    let mut world = World::new([
        Factory::new(
            Recipe::new(
                [],
                [WareAmount::new(Ware::Water, 1)],
                ProductionRate::new(1000),
            ),
            Money::from(100),
        ),
        Factory::new(
            Recipe::new([], [WareAmount::new(Ware::Seed, 1)], ProductionRate::new(1)),
            Money::from(100),
        ),
        Factory::new(
            Recipe::new(
                [
                    WareAmount::new(Ware::Water, 100),
                    WareAmount::new(Ware::Seed, 1),
                ],
                [
                    WareAmount::new(Ware::Apple, 10),
                    WareAmount::new(Ware::Seed, 2),
                ],
                ProductionRate::new(10),
            ),
            Money::from(100),
        ),
    ]);

    info!("Advancing hour");
    world.advance_hour();

    info!("World:\n{world:?}");
}
