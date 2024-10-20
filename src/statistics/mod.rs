use std::fmt::Debug;

use crate::world::World;

pub mod factory_money_statistics;

pub trait Statistics: Debug {
    fn collect(&mut self, world: &World);

    fn finalise(&self);
}
