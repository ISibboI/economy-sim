use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use crate::{
    money::ApproximateMoney,
    ware::{Ware, WareAmount},
};

#[derive(Debug, Default)]
pub struct Warehouse {
    wares: HashMap<Ware, WarehouseEntry>,
}

#[derive(Debug)]
struct WarehouseEntry {
    total_amount: u64,
    batches: VecDeque<PartialWarehouseBatch>,
}

#[derive(Debug)]
struct PartialWarehouseBatch {
    sourcing_cost_per_item: ApproximateMoney,
    amount: u64,
}

#[derive(Debug)]
pub struct WarehouseBatch {
    ware: Ware,
    amount: u64,
    sourcing_cost_per_item: ApproximateMoney,
}

impl Warehouse {
    pub fn ware_amount(&self, ware: Ware) -> WareAmount {
        match self.wares.get(&ware) {
            Some(entry) => entry.ware_amount(ware),
            None => WareAmount::new(ware, 0),
        }
    }

    pub fn insert_ware(
        &mut self,
        ware_amount: WareAmount,
        sourcing_cost_per_item: ApproximateMoney,
    ) {
        if let Some(entry) = self.wares.get_mut(&ware_amount.ware()) {
            entry.insert(ware_amount.amount(), sourcing_cost_per_item);
        } else {
            self.wares.insert(
                ware_amount.ware(),
                WarehouseEntry::new(ware_amount.amount(), sourcing_cost_per_item),
            );
        }
    }

    /// Remove the given amount of the given ware.
    ///
    /// Returns the average sourcing cost per item of the wares that were removed.
    pub fn remove_ware(&mut self, ware_amount: WareAmount) -> ApproximateMoney {
        let entry = self.wares.get_mut(&ware_amount.ware()).unwrap();
        let result = entry.remove(ware_amount.amount());
        if entry.is_empty() {
            self.wares.remove(&ware_amount.ware());
        }
        result
    }

    pub fn drain(&mut self) -> impl use<'_> + Iterator<Item = WarehouseBatch> {
        self.wares.drain().flat_map(|(ware, entry)| {
            entry
                .batches
                .into_iter()
                .map(move |partial_batch| partial_batch.into_batch(ware))
        })
    }
}

impl WarehouseEntry {
    pub fn new(amount: u64, sourcing_cost_per_item: ApproximateMoney) -> Self {
        Self {
            total_amount: amount,
            batches: [PartialWarehouseBatch {
                sourcing_cost_per_item,
                amount,
            }]
            .into_iter()
            .collect(),
        }
    }

    pub fn ware_amount(&self, ware: Ware) -> WareAmount {
        WareAmount::new(ware, self.total_amount)
    }

    pub fn is_empty(&self) -> bool {
        self.total_amount == 0
    }

    pub fn insert(&mut self, amount: u64, sourcing_cost_per_item: ApproximateMoney) {
        self.total_amount = self.total_amount.checked_add(amount).unwrap();
        self.batches.push_back(PartialWarehouseBatch {
            sourcing_cost_per_item,
            amount,
        });
    }

    pub fn remove(&mut self, mut amount: u64) -> ApproximateMoney {
        assert!(amount > 0);
        self.total_amount = self.total_amount.checked_sub(amount).unwrap();
        let mut total_sourcing_cost = ApproximateMoney::ZERO;
        let total_removed_amount = amount;

        while amount > 0 {
            let batch = self.batches.front_mut().unwrap();
            let sourcing_cost_per_item = batch.sourcing_cost_per_item;

            let batch_amount = if amount >= batch.amount {
                amount -= batch.amount;
                self.batches.pop_front().unwrap().amount
            } else {
                let batch_amount = amount;
                batch.amount -= batch_amount;
                amount = 0;
                batch_amount
            };

            total_sourcing_cost += sourcing_cost_per_item * batch_amount;
        }

        total_sourcing_cost / total_removed_amount
    }
}

impl PartialWarehouseBatch {
    pub fn into_batch(self, ware: Ware) -> WarehouseBatch {
        WarehouseBatch {
            ware,
            amount: self.amount,
            sourcing_cost_per_item: self.sourcing_cost_per_item,
        }
    }
}

impl WarehouseBatch {
    pub fn ware(&self) -> Ware {
        self.ware
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn sourcing_cost_per_item(&self) -> ApproximateMoney {
        self.sourcing_cost_per_item
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Warehouse {{")?;
        let mut once = true;
        for (ware, entry) in &self.wares {
            if once {
                once = false;
            } else {
                write!(f, ", ")?;
            }

            let ware_amount = WareAmount::new(*ware, entry.total_amount);
            write!(f, "{ware_amount}")?;
        }
        write!(f, "}}")
    }
}
