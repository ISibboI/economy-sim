use crate::{market::Market, money::Money, ware::WareAmount};

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
        todo!()
    }
}
