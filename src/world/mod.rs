use std::mem;

use general_stable_vec::{implementation::option_vec::OptionStableVec, interface::StableVec};
use log::debug;
use rand::Rng;

use crate::{
    consumer::Consumer,
    factory::{Factory, FactoryId},
    market::Market,
    statistics::Statistics,
    time::DateTime,
};

#[derive(Debug)]
pub struct World {
    factories: OptionStableVec<Factory, FactoryId>,
    consumers: Vec<Consumer>,
    market: Market,
    time: DateTime,
    statistics: Vec<Box<dyn Statistics>>,
}

impl World {
    pub fn new(
        factories: impl IntoIterator<Item = Factory>,
        consumers: impl IntoIterator<Item = Consumer>,
        statistics: Vec<Box<dyn Statistics>>,
    ) -> Self {
        Self {
            factories: factories.into_iter().collect(),
            consumers: consumers.into_iter().collect(),
            market: Default::default(),
            time: DateTime::ZERO,
            statistics,
        }
    }

    pub fn factories(&self) -> impl Iterator<Item = (FactoryId, &Factory)> {
        self.factories.iter()
    }

    pub fn time(&self) -> DateTime {
        self.time
    }

    pub fn advance_hour(&mut self, rng: &mut impl Rng) {
        debug!("Advancing world by one hour");

        // Collect initial statistics on first update.
        if self.time == DateTime::ZERO {
            self.collect_statistics();
        }

        // Advance time.
        self.time.increment();

        // Update
        // 1. Factories produce if possible.
        for factory in self.factories.iter_elements_mut() {
            factory.produce_one_hour();
        }

        // 2. Outputs get offered on the market, or reused as inputs.
        for (factory_id, factory) in self.factories.iter_mut() {
            factory.reuse_inputs();
            factory.offer_outputs(&mut self.market, factory_id);
        }

        self.market.sort_offers(rng);
        debug!("{}", self.market);

        // 3. Inputs are bought from the market (in random order).
        //    First, factories buy required inputs, and then consumers consume.
        for factory in self.factories.iter_elements_mut() {
            factory.buy_inputs(&mut self.market);
        }
        for consumer in &self.consumers {
            consumer.consume(&mut self.market);
        }

        // 4. Money is returned from the market to the factories.
        for (factory_id, factory) in self.factories.iter_mut() {
            factory.collect_money(&mut self.market, factory_id);
        }

        // 5. Collect statistics.
        self.collect_statistics();
    }

    fn collect_statistics(&mut self) {
        debug!("Collecting statistics");
        let mut statistics = mem::take(&mut self.statistics);
        for statistics in &mut statistics {
            statistics.collect(self);
        }
        self.statistics = statistics;
    }

    pub fn advance_time(&mut self, time: DateTime, rng: &mut impl Rng) {
        for _ in 0..time.into_hours() {
            self.advance_hour(rng);
        }
    }

    pub fn finalise_statistics(&self) {
        for statistics in &self.statistics {
            statistics.finalise();
        }
    }
}
