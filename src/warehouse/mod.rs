use std::collections::HashMap;

use crate::{
    money::Money,
    ware::{Ware, WareAmount},
};

#[derive(Debug, Default)]
pub struct Warehouse {
    wares: HashMap<Ware, WarehouseEntry>,
}

#[derive(Debug)]
pub struct WarehouseEntry {
    total_amount: u64,
    sourcing_costs: Vec<SourcingCost>,
}

#[derive(Debug)]
pub struct SourcingCost {
    amount: u64,
    total_cost: Money,
}

impl Warehouse {
    pub fn ware_amount(&self, ware: Ware) -> WareAmount {
        match self.wares.get(&ware) {
            Some(entry) => entry.ware_amount(ware),
            None => WareAmount::new(ware, 0),
        }
    }
}

impl WarehouseEntry {
    pub fn ware_amount(&self, ware: Ware) -> WareAmount {
        WareAmount::new(ware, self.total_amount)
    }
}
