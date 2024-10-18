use std::{
    fmt::Display,
    ops::{Mul, Rem},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(u64);

impl Money {
    pub const ZERO: Money = Money(0);

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}€", self.0)
    }
}

impl From<u64> for Money {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Mul<u64> for Money {
    type Output = Money;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Rem<u64> for Money {
    type Output = Money;

    fn rem(self, rhs: u64) -> Self::Output {
        Money(self.0 % rhs)
    }
}
