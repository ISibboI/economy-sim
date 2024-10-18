use crate::{factory::FactoryId, money::Money, ware::WareAmount};

#[derive(Debug, Default)]
pub struct Market {
    offers: Vec<MarketOffer>,
}

#[derive(Debug)]
pub struct MarketOffer {
    source_factory: FactoryId,
    wares: WareAmount,
    price_per_item: Money,
}
