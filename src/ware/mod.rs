use std::{
    fmt::Display,
    ops::{Div, Mul},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ware {
    Water,
    Seed,
    Apple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WareAmount {
    ware: Ware,
    amount: u64,
}

impl Ware {
    /// Returns true if this ware is countable in the English grammatical sense.
    fn is_countable(&self) -> bool {
        match self {
            Self::Water => false,
            Self::Seed | Self::Apple => true,
        }
    }
}

impl WareAmount {
    pub fn new(ware: Ware, amount: u64) -> Self {
        Self { ware, amount }
    }

    pub fn ware(&self) -> Ware {
        self.ware
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}

impl Mul<u64> for WareAmount {
    type Output = WareAmount;

    fn mul(mut self, rhs: u64) -> Self::Output {
        self.amount = self.amount.checked_mul(rhs).unwrap();
        self
    }
}

impl Mul<u64> for &'_ WareAmount {
    type Output = WareAmount;

    fn mul(self, rhs: u64) -> Self::Output {
        *self * rhs
    }
}

impl Mul<u64> for &'_ mut WareAmount {
    type Output = WareAmount;

    fn mul(self, rhs: u64) -> Self::Output {
        *self * rhs
    }
}

impl Div for WareAmount {
    type Output = u64;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.ware, rhs.ware);
        self.amount.checked_div(rhs.amount).unwrap()
    }
}

impl Display for Ware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ware::Water => write!(f, "water"),
            Ware::Seed => write!(f, "seed"),
            Ware::Apple => write!(f, "apple"),
        }
    }
}

impl Display for WareAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{}",
            self.amount,
            self.ware,
            if self.amount != 1 && self.ware.is_countable() {
                "s"
            } else {
                ""
            },
        )
    }
}
