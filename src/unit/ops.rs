use crate::Number;
use super::{Unit, UnitNumber};
use std::{cmp::Ordering, ops::{Add, Div, Mul, Neg, Sub}};

// Output = Self * Rhs
pub trait UnitMul<Rhs: Unit>: Unit {
    type Output: Unit;
}

// Ouptut = Self / Rhs
pub trait UnitDiv<Rhs: Unit>: Unit {
    type Output: Unit;
}

impl<LhsUnit, RhsUnit> Mul<UnitNumber<RhsUnit>> for UnitNumber<LhsUnit>
where
    LhsUnit: Unit + UnitMul<RhsUnit>,
    RhsUnit: Unit,
    <LhsUnit as UnitMul<RhsUnit>>::Output: Unit,
{
    type Output = UnitNumber<<LhsUnit as UnitMul<RhsUnit>>::Output>;

    fn mul(self, rhs: UnitNumber<RhsUnit>) -> Self::Output {
        let result = self.number * rhs.number;
        UnitNumber::new(result)
    }
}

impl<LhsUnit, RhsUnit> Div<UnitNumber<RhsUnit>> for UnitNumber<LhsUnit>
where
    LhsUnit: Unit + UnitDiv<RhsUnit>,
    RhsUnit: Unit,
    <LhsUnit as UnitDiv<RhsUnit>>::Output: Unit,
{
    type Output = UnitNumber<<LhsUnit as UnitDiv<RhsUnit>>::Output>;

    fn div(self, rhs: UnitNumber<RhsUnit>) -> Self::Output {
        let result = self.number / rhs.number;
        UnitNumber::new(result)
    }
}

// UnitNumber<U> = UnitNumber<U> + UnitNumber<U>
impl<U: Unit> Add<UnitNumber<U>> for UnitNumber<U> {
    type Output = UnitNumber<U>;
    fn add(self, rhs: UnitNumber<U>) -> Self::Output {
        let n = self.number + rhs.number;
        Self::Output::new(n)
    }
}

// UnitNumber<U> = UnitNumber<U> - UnitNumber<U>
impl<U: Unit> Sub<UnitNumber<U>> for UnitNumber<U> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let n = self.number - rhs.number;
        Self::Output::new(n)
    }
}

// UnitNumber<U> = - UnitNumber<U>
impl<U: Unit> Neg for UnitNumber<U> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        UnitNumber::new(self.number.neg())
    }
}

// UnitNumber<U> = UnitNumber<U> * Number
impl<U: Unit> Mul<Number> for UnitNumber<U> {
    type Output = Self;
    fn mul(self, rhs: Number) -> Self {
        let lhs_val = self.number.to_f64();
        let rhs_val = rhs.to_f64();
        Self::new(Number::from_f64(lhs_val * rhs_val))
    }
}

// UnitNumber<U> = Number * UnitNumber<U>
impl<U: Unit> Mul<UnitNumber<U>> for Number {
    type Output = UnitNumber<U>;
    fn mul(self, rhs: UnitNumber<U>) -> UnitNumber<U> {
        rhs * self
    }
}

// UnitNumber<U> = UnitNumber<U> / Number
impl<U: Unit> Div<Number> for UnitNumber<U> {
    type Output = Self;
    fn div(self, rhs: Number) -> Self {
        let lhs_val = self.number.to_f64();
        let rhs_val = rhs.to_f64();
        Self::new(Number::from_f64(lhs_val / rhs_val))
    }
}

//==================== Cmp and Eq =========================//

impl<U: Unit> PartialOrd for UnitNumber<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl<U: Unit> Ord for UnitNumber<U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

