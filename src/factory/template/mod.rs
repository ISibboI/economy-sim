use crate::{
    market::Market,
    money::{ApproximateMoney, Money},
    recipe::Recipe,
};

#[derive(Debug)]
pub struct FactoryTemplate {
    recipe: Recipe,
    hourly_wages: Money,
}

#[derive(Debug)]
pub enum EstimatedProfitMargin {
    /// The ratio between estimated income and estimated expenses.
    ///
    /// A margin of 1.0 means that income is equal to expenses.
    /// Lower than 1.0 means that the factory would operate at a loss,
    /// higher than 1.0 means that the factory would run at a profit.
    Margin(f64),
    MissingInput,
    MissingOutput,
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

    pub fn estimated_profit_margin(&self, market: &Market) -> EstimatedProfitMargin {
        if self
            .recipe
            .inputs()
            .iter()
            .any(|input| market.current_price(input.ware()).is_none())
        {
            return EstimatedProfitMargin::MissingInput;
        }

        if self
            .recipe
            .outputs()
            .iter()
            .any(|output| market.current_price(output.ware()).is_none())
        {
            return EstimatedProfitMargin::MissingOutput;
        }

        let hourly_expenses = self.hourly_wages
            + self
                .recipe
                .inputs()
                .iter()
                .map(|input| market.current_price(input.ware()).unwrap() * input.amount())
                .sum::<Money>()
                * self.recipe.rate().per_hour();

        let hourly_income = self
            .recipe
            .outputs()
            .iter()
            .map(|output| market.current_price(output.ware()).unwrap() * output.amount())
            .sum::<Money>()
            * self.recipe.rate().per_hour();

        EstimatedProfitMargin::Margin(
            ApproximateMoney::from(hourly_income) / ApproximateMoney::from(hourly_expenses),
        )
    }
}
