use std::collections::{HashMap, VecDeque};

use crate::{
    money::ApproximateMoney,
    ware::{Ware, WareAmount},
};

#[derive(Debug, Default)]
pub struct Warehouse {
    wares: HashMap<Ware, WarehouseEntry>,
}

#[derive(Debug)]
pub struct WarehouseEntry {
    total_amount: u64,
    batches: VecDeque<WarehouseBatch>,
}

#[derive(Debug)]
pub struct WarehouseBatch {
    sourcing_cost_per_item: ApproximateMoney,
    amount: u64,
}

#[derive(Debug)]
pub struct WareGroupRemoveIterator<'warehouse, 'group> {
    warehouse: &'warehouse mut Warehouse,
    group: &'group [WareAmount],
    amount: u64,
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

    pub fn remove_ware(&mut self, ware_amount: WareAmount) -> ApproximateMoney {
        let entry = self.wares.get_mut(&ware_amount.ware()).unwrap();
        let result = entry.remove(ware_amount.amount());
        if entry.is_empty() {
            self.wares.remove(&ware_amount.ware());
        }
        result
    }
}

impl WarehouseEntry {
    pub fn new(amount: u64, sourcing_cost_per_item: ApproximateMoney) -> Self {
        Self {
            total_amount: amount,
            batches: [WarehouseBatch {
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
        self.batches.push_back(WarehouseBatch {
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
