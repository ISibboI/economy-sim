use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, Rem, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(u64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ApproximateMoney(f64);

impl Money {
    pub const ZERO: Money = Money(0);

    pub fn raw(&self) -> u64 {
        self.0
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
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

impl From<Money> for u64 {
    fn from(value: Money) -> Self {
        value.0
    }
}

impl From<Money> for f64 {
    fn from(value: Money) -> Self {
        value.0 as f64
    }
}

impl Mul<u64> for Money {
    type Output = Money;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Money> for u64 {
    type Output = Money;

    fn mul(self, rhs: Money) -> Self::Output {
        rhs * self
    }
}

impl Div<u64> for Money {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0.checked_div(rhs).unwrap())
    }
}

impl Rem<u64> for Money {
    type Output = Money;

    fn rem(self, rhs: u64) -> Self::Output {
        Self(self.0 % rhs)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.checked_add(rhs.0).unwrap();
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.checked_sub(rhs.0).unwrap()
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_add(rhs.0).unwrap())
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_sub(rhs.0).unwrap())
    }
}

impl Div for Money {
    type Output = u64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.checked_div(rhs.0).unwrap()
    }
}

impl From<f64> for ApproximateMoney {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<u64> for ApproximateMoney {
    fn from(value: u64) -> Self {
        Self(value as f64)
    }
}

impl From<ApproximateMoney> for f64 {
    fn from(value: ApproximateMoney) -> Self {
        value.0
    }
}

impl From<Money> for ApproximateMoney {
    fn from(value: Money) -> Self {
        Self(value.0 as f64)
    }
}

impl Div for ApproximateMoney {
    type Output = ApproximateMoney;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
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

impl Add<f64> for ApproximateMoney {
    type Output = ApproximateMoney;

    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign for ApproximateMoney {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl DivAssign<u64> for ApproximateMoney {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs as f64
    }
}

impl Display for ApproximateMoney {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.0.is_sign_positive(), "{}", self.0);
        assert!(self.0.is_normal() || self.0 == 0.0, "{}", self.0);

        if self.0 < 1e0 {
            write!(f, "{:.3}€", self.0)
        } else if self.0 < 1e1 {
            write!(f, "{:.2}€", self.0)
        } else if self.0 < 1e2 {
            write!(f, "{:.1}€", self.0)
        } else if self.0 < 1e3 {
            write!(f, "{:.0}€", self.0)
        } else if self.0 < 1e4 {
            write!(f, "{:.2}k€", self.0)
        } else {
            write!(f, "{:.1}k€", self.0)
        }
    }
}
