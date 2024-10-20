use log::debug;

use crate::{
    market::Market,
    money::{ApproximateMoney, Money},
    ware::WareAmount,
};

#[derive(Debug)]
pub struct Consumer {
    target_ware_amount: WareAmount,
    target_price: Money,
}

impl Consumer {
    pub fn new(target_ware_amount: WareAmount, target_price: Money) -> Self {
        Self {
            target_ware_amount,
            target_price,
        }
    }

    pub fn consume(&self, market: &mut Market) {
        let mut demand = self.target_ware_amount.amount() as f64;
        let mut total_consumption = 0;
        let mut average_price = ApproximateMoney::ZERO;

        while demand > 0.0 {
            let Some(price) = market.current_price(self.target_ware_amount.ware()) else {
                break;
            };

            let demand_per_item = (price.raw() as f64 / self.target_price.raw() as f64).sqrt();
            let buy_amount = (demand / demand_per_item).round() as u64;
            if buy_amount == 0 {
                break;
            }

            let consume_amount = self.target_ware_amount.with_amount(buy_amount);
            let consumed_amount = market.consume_at_current_price(consume_amount);
            demand -= demand_per_item * consumed_amount as f64;
            total_consumption += consumed_amount;
            average_price += (consumed_amount * price).into();
        }

        average_price /= total_consumption;

        if total_consumption > 0 {
            debug!(
                "Consumed {} at average price {}",
                self.target_ware_amount.with_amount(total_consumption),
                average_price,
            );
        }
    }
}
