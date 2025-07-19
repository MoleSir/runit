mod units;
mod ops;

use core::fmt;
use std::marker::PhantomData;
use crate::Number;
pub use units::*;
pub use ops::*;

pub trait Unit : PartialEq + Eq + Clone + Copy {
    fn name() -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitNumber<U> {
    number: Number,
    unit: PhantomData<U>,
}

impl<U: Unit> UnitNumber<U> {
    pub fn new<N: Into<Number>>(number: N) -> Self {
        Self { number: number.into(), unit: PhantomData }
    }

    pub fn to_f64(&self) -> f64 {
        self.number.to_f64()
    }

    pub fn value(&self) -> Number {
        self.number
    }
}

impl<U: Unit> fmt::Display for UnitNumber<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "{:.*}{}", p, self.number, U::name())
        } else {
            write!(f, "{}{}", self.number, U::name())
        }
    }
}

