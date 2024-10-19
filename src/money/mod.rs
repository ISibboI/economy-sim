use std::{
    fmt::Display,
    ops::{AddAssign, Div, Mul, Rem},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(u64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ApproximateMoney(f64);

impl Money {
    pub const ZERO: Money = Money(0);

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }
}

impl ApproximateMoney {
    pub const ZERO: ApproximateMoney = ApproximateMoney(0.0);
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
        Self(self.0 % rhs)
    }
}

impl From<f64> for ApproximateMoney {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<ApproximateMoney> for Money {
    fn from(value: ApproximateMoney) -> Self {
        Self(value.0.round() as u64)
    }
}

impl From<Money> for ApproximateMoney {
    fn from(value: Money) -> Self {
        Self(value.0 as f64)
    }
}

impl Mul<u64> for ApproximateMoney {
    type Output = ApproximateMoney;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs as f64)
    }
}

impl Div<u64> for ApproximateMoney {
    type Output = ApproximateMoney;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs as f64)
    }
}

impl AddAssign for ApproximateMoney {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}
