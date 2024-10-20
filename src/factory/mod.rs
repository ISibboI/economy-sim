use std::fmt::Display;

use general_stable_vec::interface::StableVecIndex;
use log::debug;
use template::FactoryTemplate;

use crate::{
    market::Market,
    money::{ApproximateMoney, Money},
    time::DateTime,
    ware::WareAmount,
    warehouse::Warehouse,
};

pub mod template;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactoryId(usize);

#[derive(Debug)]
pub struct Factory {
    template: FactoryTemplate,
    input_storage: Warehouse,
    output_storage: Warehouse,
    money: Money,
}

impl Factory {
    pub fn new(template: FactoryTemplate, starting_money: Money) -> Self {
        Self {
            template,
            input_storage: Default::default(),
            output_storage: Default::default(),
            money: starting_money,
        }
    }

    pub fn money(&self) -> Money {
        self.money
    }

    pub fn produce_one_hour(&mut self) {
        self.produce(DateTime::from_hours(1));
    }

    pub fn produce(&mut self, duration: DateTime) {
        debug!(
            "Factory with recipe {} produces for {duration} with {} and inputs {}",
            self.template.recipe(),
            self.money,
            self.input_storage
        );

        // Compute available recipe applications.
        let maximum_recipe_application_amount = self.template.recipe().rate() * duration;
        let recipe_application_amount = self
            .template
            .recipe()
            .inputs()
            .iter()
            .copied()
            .fold(
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
            )
            .min(self.money / self.template.hourly_wages());
        debug!("Executing the recipe {recipe_application_amount} times");

        if recipe_application_amount > 0 {
            let duration =
                recipe_application_amount.div_ceil(self.template.recipe().rate().per_hour());
            let wages = self.template.hourly_wages() * duration;
            self.money -= wages;
            let mut sourcing_cost_per_item =
                ApproximateMoney::from(wages) / recipe_application_amount;

            // Apply recipe.
            for input in self.template.recipe().inputs() {
                sourcing_cost_per_item += self
                    .input_storage
                    .remove_ware(input * recipe_application_amount)
                    * input.amount();
            }

            sourcing_cost_per_item /= self.template.recipe().output_amount();
            for output in self.template.recipe().outputs() {
                self.output_storage
                    .insert_ware(*output * recipe_application_amount, sourcing_cost_per_item);
            }
        }
    }

    pub fn reuse_inputs(&mut self) {
        let recipe_production_per_hour = self.template.recipe().rate().per_hour();

        for input in self.template.recipe().inputs() {
            let required_amount = input.amount() * recipe_production_per_hour;
            let available_amount = self.input_storage.ware_amount(input.ware()).amount();
            let missing_amount = required_amount.saturating_sub(available_amount);
            let reusable_amount = self.output_storage.ware_amount(input.ware()).amount();
            let transfer_amount =
                WareAmount::new(input.ware(), missing_amount.min(reusable_amount));

            if transfer_amount.amount() > 0 {
                let sourcing_cost_per_item = self.output_storage.remove_ware(transfer_amount);
                self.input_storage
                    .insert_ware(transfer_amount, sourcing_cost_per_item);
            }
        }
    }

    pub fn offer_outputs(&mut self, market: &mut Market, factory_id: FactoryId) {
        for batch in self.output_storage.drain() {
            market.offer(
                batch.ware(),
                batch.amount(),
                (f64::from(batch.sourcing_cost_per_item() + 0.5).ceil() as u64).into(),
                factory_id,
            );
        }
    }

    pub fn buy_inputs(&mut self, market: &mut Market) {
        if self.template.recipe().inputs().is_empty() {
            return;
        }

        debug!(
            "Buying factory inputs from market for recipe {}",
            self.template.recipe()
        );

        let recipe_production_per_hour = self.template.recipe().rate().per_hour();

        let mut left = 0;
        let mut right = recipe_production_per_hour;
        let mut ceil_middle = false;

        while left < right {
            let middle = if ceil_middle {
                (left + right).div_ceil(2)
            } else {
                (left + right) / 2
            };
            ceil_middle = !ceil_middle;

            let mut total_price = Money::ZERO;

            for input in self.template.recipe().inputs() {
                let required_amount = input.amount() * middle;
                let available_amount = self.input_storage.ware_amount(input.ware()).amount();
                let missing_amount = required_amount.saturating_sub(available_amount);
                let (_, price) = market.total_price(WareAmount::new(input.ware(), missing_amount));
                total_price += price;
            }

            if total_price <= self.money.saturating_sub(self.template.hourly_wages()) {
                left = middle;
            } else {
                right = middle - 1;
            }
        }

        debug_assert_eq!(left, right);
        let buy_target = left;

        for input in self.template.recipe().inputs() {
            let required_amount = input.amount() * buy_target;
            let available_amount = self.input_storage.ware_amount(input.ware()).amount();
            let missing_amount = required_amount.saturating_sub(available_amount);

            let ware_amount = WareAmount::new(input.ware(), missing_amount);
            let actual_amount = market.buy(ware_amount, &mut self.input_storage, &mut self.money);
            let actual_ware_amount = ware_amount.with_amount(actual_amount);
            debug!("Bought {actual_ware_amount} (wanted {missing_amount})");
        }
    }

    pub fn collect_money(&mut self, market: &mut Market, factory_id: FactoryId) {
        market.transfer_money(&mut self.money, factory_id);
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

impl Display for FactoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
