use std::{
    fmt::Display,
    ops::{Add, Div, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    hours: u64,
}

impl DateTime {
    pub const ZERO: Self = DateTime::from_hours(0);

    pub const fn from_hours(hours: u64) -> Self {
        Self { hours }
    }

    pub fn increment(&mut self) {
        self.hours = self.hours.checked_add(1).unwrap();
    }

    pub fn into_hours(self) -> u64 {
        self.hours
    }

    pub fn saturating_sub(&self, rhs: Self) -> Self {
        Self {
            hours: self.hours.saturating_sub(rhs.hours),
        }
    }
}

impl Add for DateTime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            hours: self.hours.checked_add(rhs.hours).unwrap(),
        }
    }
}

impl Sub for DateTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            hours: self.hours.checked_sub(rhs.hours).unwrap(),
        }
    }
}

impl Div<u64> for DateTime {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self {
            hours: self.hours.checked_div(rhs).unwrap(),
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} hours", self.hours)
    }
}
