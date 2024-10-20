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
    fulfilment: f64,
    decay: f64,
}

impl Consumer {
    pub fn new(target_ware_amount: WareAmount, target_price: Money, decay: f64) -> Self {
        assert!(decay.is_normal() || decay == 0.0);
        assert!(decay >= 0.0);
        assert!(decay <= 1.0);

        Self {
            target_ware_amount,
            target_price,
            fulfilment: 1.0,
            decay,
        }
    }

    pub fn consume(&mut self, market: &mut Market) {
        assert!(self.fulfilment >= 0.0);
        assert!(self.fulfilment.is_normal() || self.fulfilment == 0.0);

        let raw_demand = self.target_ware_amount.amount() as f64;
        let mut demand = raw_demand
            / if self.fulfilment < 1.0 {
                self.fulfilment * 0.5 + 0.5
            } else {
                self.fulfilment
            };
        let extra_demand = demand - raw_demand;
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

        let unfulfilled_demand = (demand - extra_demand) / raw_demand;
        println!("unfulfilled_demand: {unfulfilled_demand:.2}");
        self.fulfilment -= (1.0 - self.decay) * unfulfilled_demand;
        self.fulfilment -= (self.fulfilment - 1.0) * (1.0 - self.decay);

        average_price /= total_consumption;

        if total_consumption > 0 {
            debug!(
                "Consumed {} at average price {} (fulfilment: {:.2})",
                self.target_ware_amount.with_amount(total_consumption),
                average_price,
                self.fulfilment,
            );
        }
    }
}
