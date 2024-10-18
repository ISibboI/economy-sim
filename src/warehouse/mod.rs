use std::collections::{BTreeMap, HashMap};

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
    batches: Vec<WarehouseBatch>,
}

#[derive(Debug)]
pub struct WarehouseBatch {
    sourcing_cost_per_item: Money,
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

    pub fn insert_ware(&mut self, ware_amount: WareAmount, sourcing_cost_per_item: Money) {
        if let Some(entry) = self.wares.get_mut(&ware_amount.ware()) {
            entry.insert(ware_amount.amount(), sourcing_cost_per_item);
        } else {
            self.wares.insert(
                ware_amount.ware(),
                WarehouseEntry::new(ware_amount.amount(), sourcing_cost_per_item),
            );
        }
    }

    pub fn remove_ware_groups<'this, 'group>(
        &'this mut self,
        group: &'group [WareAmount],
        amount: u64,
    ) -> impl Iterator<Item = WarehouseBatch> + use<'this, 'group> {
        WareGroupRemoveIterator {
            warehouse: self,
            group,
            amount,
        }
    }
}

impl WarehouseEntry {
    pub fn new(amount: u64, sourcing_cost_per_item: Money) -> Self {
        Self {
            total_amount: amount,
            batches: vec![WarehouseBatch {
                sourcing_cost_per_item,
                amount,
            }],
        }
    }

    pub fn ware_amount(&self, ware: Ware) -> WareAmount {
        WareAmount::new(ware, self.total_amount)
    }

    pub fn is_empty(&self) -> bool {
        self.total_amount == 0
    }

    pub fn insert(&mut self, amount: u64, sourcing_cost_per_item: Money) {
        self.total_amount = self.total_amount.checked_add(amount).unwrap();
        self.batches.push(WarehouseBatch {
            sourcing_cost_per_item,
            amount,
        });
    }
}

impl<'warehouse, 'group> Iterator for WareGroupRemoveIterator<'warehouse, 'group> {
    type Item = WarehouseBatch;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
