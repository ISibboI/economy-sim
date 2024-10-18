use crate::wares::{Ware, WareAmount};

pub struct Recipe {
    inputs: Vec<WareAmount>,
    outputs: Vec<WareAmount>,
    rate: ProductionRate,
}

pub struct ProductionRate {
    per_hour: u64,
}
