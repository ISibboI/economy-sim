use consumer::Consumer;
use factory::Factory;
use log::info;
use money::Money;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use recipe::{ProductionRate, Recipe};
use simplelog::TermLogger;
use statistics::factory_money_statistics::FactoryMoneyStatistics;
use time::DateTime;
use ware::{Ware, WareAmount};
use world::World;

mod consumer;
mod factory;
mod market;
mod money;
mod recipe;
mod statistics;
mod time;
mod ware;
mod warehouse;
mod world;

fn main() {
    TermLogger::init(
        log::LevelFilter::Debug,
        Default::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    info!("Creating world");
    let mut world = World::new(
        [
            Factory::new(
                Recipe::new(
                    [],
                    [WareAmount::new(Ware::Water, 10)],
                    ProductionRate::new(100),
                ),
                Money::from(100),
                Money::from(10_000),
            ),
            Factory::new(
                Recipe::new([], [WareAmount::new(Ware::Seed, 1)], ProductionRate::new(1)),
                Money::from(100),
                Money::from(10_000),
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
                Money::from(10_000),
            ),
        ],
        [Consumer::new(
            WareAmount::new(Ware::Apple, 10),
            Money::from(2),
        )],
        vec![Box::new(FactoryMoneyStatistics::new("factory_money.svg"))],
    );

    info!("Creating rng");
    let mut rng = Xoshiro256PlusPlus::from_entropy();

    info!("Advancing hour");
    world.advance_time(DateTime::from_hours(10), &mut rng);

    info!("Finalising statistics");
    world.finalise_statistics();
}
