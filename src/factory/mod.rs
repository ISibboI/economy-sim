use general_stable_vec::interface::StableVecIndex;

use crate::{
    money::{ApproximateMoney, Money},
    recipe::Recipe,
    time::DateTime,
    warehouse::Warehouse,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactoryId(usize);

#[derive(Debug)]
pub struct Factory {
    recipe: Recipe,
    input_storage: Warehouse,
    output_storage: Warehouse,
    hourly_wages: Money,
    money: Money,
}

impl Factory {
    pub fn new(recipe: Recipe, hourly_wages: Money) -> Self {
        assert!(hourly_wages % recipe.rate().per_hour() == Money::ZERO);
        Self {
            recipe,
            input_storage: Default::default(),
            output_storage: Default::default(),
            hourly_wages,
            money: Money::ZERO,
        }
    }

    pub fn produce_one_hour(&mut self) {
        self.produce(DateTime::from_hours(1));
    }

    pub fn produce(&mut self, duration: DateTime) {
        // Compute available recipe applications.
        let maximum_recipe_application_amount = self.recipe.rate() * duration;
        let recipe_application_amount = self.recipe.inputs().iter().copied().fold(
            maximum_recipe_application_amount,
            |recipe_application_amount, single_input_amount| {
                let required_input_amount = single_input_amount * recipe_application_amount;
                let available_input_amount =
                    self.input_storage.ware_amount(required_input_amount.ware());

                if available_input_amount < required_input_amount {
                    available_input_amount / single_input_amount
                } else {
                    recipe_application_amount
                }
            },
        );
        let duration = recipe_application_amount.div_ceil(self.recipe.rate().per_hour());
        let wages = self.hourly_wages * duration;
        let mut sourcing_cost_per_item = ApproximateMoney::from(wages) / recipe_application_amount;

        // Apply recipe.
        for input in self.recipe.inputs() {
            sourcing_cost_per_item += self
                .input_storage
                .remove_ware(input * recipe_application_amount);
        }
        for output in self.recipe.outputs() {
            self.output_storage
                .insert_ware(*output * recipe_application_amount, sourcing_cost_per_item);
        }
    }
}

impl StableVecIndex for FactoryId {}

impl From<usize> for FactoryId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<FactoryId> for usize {
    fn from(value: FactoryId) -> Self {
        value.0
    }
}
