use std::{fmt::Display, ops::Mul};

use crate::{time::DateTime, ware::WareAmount};

#[derive(Debug)]
pub struct Recipe {
    inputs: Vec<WareAmount>,
    outputs: Vec<WareAmount>,
    rate: ProductionRate,
}

#[derive(Debug, Clone, Copy)]
pub struct ProductionRate {
    per_hour: u64,
}

impl Recipe {
    pub fn new(
        inputs: impl IntoIterator<Item = WareAmount>,
        outputs: impl IntoIterator<Item = WareAmount>,
        rate: ProductionRate,
    ) -> Self {
        Self {
            inputs: inputs.into_iter().collect(),
            outputs: outputs.into_iter().collect(),
            rate,
        }
    }

    pub fn rate(&self) -> ProductionRate {
        self.rate
    }

    pub fn inputs(&self) -> &[WareAmount] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[WareAmount] {
        &self.outputs
    }
}

impl ProductionRate {
    pub fn new(per_hour: u64) -> Self {
        Self { per_hour }
    }

    pub fn per_hour(&self) -> u64 {
        self.per_hour
    }
}

impl Mul<DateTime> for ProductionRate {
    type Output = u64;

    fn mul(self, rhs: DateTime) -> Self::Output {
        self.per_hour * rhs.into_hours()
    }
}

impl Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Recipe {{rate: {}, recipe: (", self.rate)?;
        let mut once = true;
        for input in &self.inputs {
            if once {
                once = false;
            } else {
                write!(f, ", ")?;
            }

            write!(f, "{input}")?;
        }
        write!(f, ") -> (")?;
        once = true;
        for output in &self.outputs {
            if once {
                once = false;
            } else {
                write!(f, ", ")?;
            }

            write!(f, "{output}")?;
        }
        write!(f, ")}}")
    }
}

impl Display for ProductionRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/h", self.per_hour)
    }
}
