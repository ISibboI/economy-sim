#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    hours: u64,
}

impl DateTime {
    pub const ZERO: Self = DateTime::from_hours(0);

    pub const fn from_hours(hours: u64) -> Self {
        Self { hours }
    }

    pub fn increment(&mut self) {
        self.hours.checked_add(1).unwrap();
    }

    pub fn into_hours(self) -> u64 {
        self.hours
    }
}
