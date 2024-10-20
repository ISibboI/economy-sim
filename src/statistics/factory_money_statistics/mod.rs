use std::{collections::HashMap, path::PathBuf};

use crate::{factory::FactoryId, money::Money, time::DateTime, world::World};

use super::Statistics;

#[derive(Debug)]
pub struct FactoryMoneyStatistics {
    output_file: PathBuf,
    money_time_series: HashMap<FactoryId, Vec<(DateTime, Money)>>,
}

impl FactoryMoneyStatistics {
    pub fn new(output_file: impl Into<PathBuf>) -> Self {
        Self {
            output_file: output_file.into(),
            money_time_series: Default::default(),
        }
    }
}

impl Statistics for FactoryMoneyStatistics {
    fn collect(&mut self, world: &World) {
        for (factory_id, factory) in world.factories() {
            let entry = (world.time(), factory.money());
            if let Some(money_time_series) = self.money_time_series.get_mut(&factory_id) {
                money_time_series.push(entry);
            } else {
                self.money_time_series.insert(factory_id, vec![entry]);
            }
        }
    }

    fn finalise(&self) {
        let mut iter = self
            .money_time_series
            .values()
            .flat_map(|money_time_series| money_time_series.iter())
            .copied();
        let (first_time, first_money) = iter.next().unwrap();
        let (min_time, max_time, min_money, max_money) = iter.fold(
            (first_time, first_time, first_money, first_money),
            |(min_time, max_time, min_money, max_money), (time, money)| {
                (
                    min_time.min(time),
                    max_time.max(time),
                    min_money.min(money),
                    max_money.max(money),
                )
            },
        );

        todo!()
    }
}
