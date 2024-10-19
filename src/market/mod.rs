use std::collections::HashMap;

use rand::{seq::SliceRandom, Rng};

use crate::{
    factory::FactoryId,
    money::Money,
    ware::{Ware, WareAmount},
    warehouse::Warehouse,
};

#[derive(Debug, Default)]
pub struct Market {
    offers: HashMap<Ware, Vec<MarketOffer>>,
}

#[derive(Debug)]
struct MarketOffer {
    source_factory: FactoryId,
    amount: u64,
    price_per_item: Money,
}

impl Market {
    pub fn offer(
        &mut self,
        ware: Ware,
        amount: u64,
        price_per_item: Money,
        source_factory: FactoryId,
    ) {
        let offer = MarketOffer {
            source_factory,
            amount,
            price_per_item,
        };

        if let Some(offers) = self.offers.get_mut(&ware) {
            offers.push(offer);
        } else {
            self.offers.insert(ware, vec![offer]);
        }
    }

    /// Sort market offers by price descending.
    ///
    /// The lowest price will be at the end of the [`Vec`],
    /// and hence the cheapest offer can be bought efficiently with [`Vec::pop`].
    pub fn sort(&mut self, rng: &mut impl Rng) {
        for offers in self.offers.values_mut() {
            offers.shuffle(rng);
            offers.sort_by(|a, b| b.price_per_item.cmp(&a.price_per_item));
        }
    }

    /// Attempt to buy the requested amount of wares and store them into the output warehouse.
    ///
    /// The returned value is the actual amount bought,
    /// which may be lower in case there are not enough offers on the market.
    pub fn buy(&mut self, ware_amount: WareAmount, output_warehouse: &mut Warehouse) -> u64 {
        if let Some(offers) = self.offers.get_mut(&ware_amount.ware()) {
            todo!()
        } else {
            0
        }
    }
}
