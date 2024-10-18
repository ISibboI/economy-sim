use general_stable_vec::{implementation::option_vec::OptionStableVec, interface::StableVec};

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

    pub fn advance_hour(&mut self) {
        // Advance time
        self.time.increment();

        // 1. Factories produce if possible.
        for factory in self.factories.iter_elements_mut() {
            factory.produce_one_hour();
        }

        // 2. Outputs get offered on the market.

        // 3. Inputs are bought from the market (in random order).
    }
}
