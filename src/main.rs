use std::time::Instant;

use clap::Parser;
use consumer::Consumer;
use factory::Factory;
use log::{info, LevelFilter};
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

#[derive(Parser)]
struct Cli {
    #[arg(long, short = 'l', default_value = "Info")]
    log_level: LevelFilter,

    #[arg(long, short = 'r', default_value = "10")]
    rounds: u64,
}

fn main() {
    let cli = Cli::parse();

    TermLogger::init(
        cli.log_level,
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
            WareAmount::new(Ware::Apple, 100),
            Money::from(11),
            0.9,
        )],
        vec![Box::new(FactoryMoneyStatistics::new("factory_money.svg"))],
    );

    info!("Creating rng");
    let mut rng = Xoshiro256PlusPlus::from_entropy();

    info!("Computing {} rounds", cli.rounds);
    let start_time = Instant::now();
    world.advance_time(DateTime::from_hours(cli.rounds), &mut rng);
    let end_time = Instant::now();

    let duration = end_time - start_time;
    let duration_per_round = duration.as_secs_f64() / cli.rounds as f64;
    info!(
        "Took {}s to compute {} rounds ({}s/round)",
        duration.as_secs_f32(),
        cli.rounds,
        duration_per_round as f32
    );

    info!("Finalising statistics");
    world.finalise_statistics();

    info!("Done");
}
