use crate::{money::Money, recipe::Recipe};

#[derive(Debug)]
pub struct FactoryTemplate {
    recipe: Recipe,
    hourly_wages: Money,
}

impl FactoryTemplate {
    pub fn new(recipe: Recipe, hourly_wages: Money) -> Self {
        assert!(hourly_wages % recipe.rate().per_hour() == Money::ZERO);
        Self {
            recipe,
            hourly_wages,
        }
    }

    pub fn recipe(&self) -> &Recipe {
        &self.recipe
    }

    pub fn hourly_wages(&self) -> Money {
        self.hourly_wages
    }
}
