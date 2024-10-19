use general_stable_vec::{
    implementation::option_vec::OptionStableVec,
    interface::{StableVec, StableVecAccess},
};
use rand::Rng;

use crate::{
    factory::{Factory, FactoryId},
    market::Market,
    time::DateTime,
};

#[derive(Debug)]
pub struct World {
    factories: OptionStableVec<Factory, FactoryId>,
    market: Market,
    time: DateTime,
}

impl World {
    pub fn new(factories: impl IntoIterator<Item = Factory>) -> Self {
        Self {
            factories: factories.into_iter().collect(),
            market: Default::default(),
            time: DateTime::ZERO,
        }
    }

    pub fn advance_hour(&mut self, rng: &mut impl Rng) {
        // Advance time
        self.time.increment();

        // 1. Factories produce if possible.
        for factory in self.factories.iter_elements_mut() {
            factory.produce_one_hour();
        }

        // 2. Outputs get offered on the market.
        for (factory_id, factory) in self.factories.iter_mut() {
            factory.offer_outputs(&mut self.market, factory_id);
        }

        self.market.sort(rng);

        // 3. Inputs are bought from the market (in random order).
        for factory in self.factories.iter_elements_mut() {
            factory.buy_inputs(&mut self.market);
        }

        // 4. Money is returned from the market to the factories.
        todo!()
    }
}
