use std::{collections::HashMap, fmt::Display};

use rand::{seq::SliceRandom, Rng};

use crate::{
    factory::FactoryId,
    money::{ApproximateMoney, Money},
    ware::{Ware, WareAmount},
    warehouse::Warehouse,
};

#[derive(Debug, Default)]
pub struct Market {
    market_offers_sorted: bool,
    offers: HashMap<Ware, Vec<MarketOffer>>,
    money_transactions: HashMap<FactoryId, Vec<Money>>,
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
        self.market_offers_sorted = false;
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
    pub fn sort_offers(&mut self, rng: &mut impl Rng) {
        for offers in self.offers.values_mut() {
            offers.shuffle(rng);
            offers.sort_by(|a, b| b.price_per_item.cmp(&a.price_per_item));
        }
        self.market_offers_sorted = true;
    }

    /// Attempt to buy the requested amount of wares and store them into the output warehouse.
    ///
    /// The returned value is the actual amount bought,
    /// which may be lower in case there are not enough offers on the market.
    pub fn buy(
        &mut self,
        ware_amount: WareAmount,
        output_warehouse: &mut Warehouse,
        money: &mut Money,
    ) -> u64 {
        assert!(self.market_offers_sorted);
        let mut total_sourcing_cost = Money::ZERO;
        let bought_amount = if let Some(offers) = self.offers.get_mut(&ware_amount.ware()) {
            let mut remaining_amount = ware_amount.amount();
            while remaining_amount > 0 {
                if let Some(offer) = offers.last_mut() {
                    let money_limited_amount = *money / offer.price_per_item;
                    let offer_buy_amount =
                        offer.amount.min(remaining_amount).min(money_limited_amount);
                    if offer_buy_amount == 0 {
                        // Can't buy more with the current money.
                        break;
                    }

                    let offer_sourcing_cost = offer.price_per_item * offer_buy_amount;
                    total_sourcing_cost += offer_sourcing_cost;
                    *money -= offer_sourcing_cost;

                    if let Some(money_transactions) =
                        self.money_transactions.get_mut(&offer.source_factory)
                    {
                        money_transactions.push(offer_sourcing_cost);
                    } else {
                        self.money_transactions
                            .insert(offer.source_factory, vec![offer_sourcing_cost]);
                    }

                    offer.amount -= offer_buy_amount;
                    remaining_amount -= offer_buy_amount;

                    if offer.amount == 0 {
                        offers.pop();
                    }
                } else {
                    break;
                }
            }

            ware_amount.amount() - remaining_amount
        } else {
            0
        };

        output_warehouse.insert_ware(
            WareAmount::new(ware_amount.ware(), bought_amount),
            ApproximateMoney::from(total_sourcing_cost) / bought_amount,
        );

        bought_amount
    }

    pub fn consume_at_current_price(&mut self, ware_amount: WareAmount) -> u64 {
        assert!(self.market_offers_sorted);

        let bought_amount = if let Some(offers) = self.offers.get_mut(&ware_amount.ware()) {
            if let Some(offer) = offers.last_mut() {
                let offer_buy_amount = offer.amount.min(ware_amount.amount());
                let offer_sourcing_cost = offer.price_per_item * offer_buy_amount;

                if let Some(money_transactions) =
                    self.money_transactions.get_mut(&offer.source_factory)
                {
                    money_transactions.push(offer_sourcing_cost);
                } else {
                    self.money_transactions
                        .insert(offer.source_factory, vec![offer_sourcing_cost]);
                }

                offer.amount -= offer_buy_amount;

                if offer.amount == 0 {
                    offers.pop();
                }

                offer_buy_amount
            } else {
                0
            }
        } else {
            0
        };

        bought_amount
    }

    pub fn total_price(&mut self, ware_amount: WareAmount) -> (u64, Money) {
        assert!(self.market_offers_sorted);
        let mut total_sourcing_cost = Money::ZERO;
        let bought_amount = if let Some(offers) = self.offers.get_mut(&ware_amount.ware()) {
            let mut remaining_amount = ware_amount.amount();
            for offer in offers.iter().rev() {
                let offer_buy_amount = offer.amount.min(remaining_amount);

                let offer_sourcing_cost = offer.price_per_item * offer_buy_amount;
                total_sourcing_cost += offer_sourcing_cost;
                remaining_amount -= offer_buy_amount;

                if remaining_amount == 0 {
                    break;
                }
            }

            ware_amount.amount() - remaining_amount
        } else {
            0
        };

        (bought_amount, total_sourcing_cost)
    }

    pub fn current_price(&self, ware: Ware) -> Option<Money> {
        assert!(self.market_offers_sorted);
        match self.offers.get(&ware) {
            Some(offers) => offers.last().map(|offer| offer.price_per_item),
            None => None,
        }
    }

    pub fn transfer_money(&mut self, money: &mut Money, factory_id: FactoryId) {
        for money_transaction in self
            .money_transactions
            .remove(&factory_id)
            .unwrap_or_default()
        {
            *money += money_transaction;
        }
    }
}

impl Display for Market {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Market {{")?;
        let mut once = false;
        for (ware, offers) in &self.offers {
            if once {
                write!(f, ", ")?;
            } else {
                once = true;
            }
            write!(f, "{ware}: {}", offers.last().unwrap().price_per_item)?;
        }
        write!(f, "}}")
    }
}
