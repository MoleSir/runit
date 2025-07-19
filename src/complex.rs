use core::fmt;
use std::ops::{Add, Div, Mul, Sub};

use crate::Number;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: Number,
    pub im: Number,
}

impl Complex {
    pub fn new<N1: Into<Number>, N2: Into<Number>>(re: N1, im: N2) -> Self {
        Self { re: re.into(), im: im.into() }
    }

    pub fn parts(&self) -> (Number, Number) {
        (self.re, self.im)
    }

    pub fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn norm_sqr(self) -> Number {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        Complex::new(a + c, b + d)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        Complex::new(a - c, b - d)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        // (a + bj) * (c + dj) = (ac - bd) + (ad + bc)j
        Complex::new(a * c - b * d, a * d + b * c)
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let (a, b) = self.parts();
        let (c, d) = rhs.parts();
        let denom = c * c + d * d;
        if denom.is_zero() {
            panic!("Divide by zero in complex division");
        }
        let re = (a * c + b * d) / denom;
        let im = (b * c - a * d) / denom;
        Complex::new(re, im)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            if self.im >= 0.0 {
                write!(f, "{:.*} + {:.*}j", p, self.re, p, self.im)
            } else {
                write!(f, "{:.*} - {:.*}j", p, self.re, p, -self.im)
            }
        } else {
            if self.im >= 0.0 {
                write!(f, "{} + {}j", self.re, self.im)
            } else {
                write!(f, "{} - {}j", self.re, -self.im)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(val: f64) -> Number {
        Number::from(val)
    }

    #[test]
    fn test_creation() {
        let c = Complex { re: n(3.0), im: n(4.0) };
        assert_eq!(c.re, n(3.0));
        assert_eq!(c.im, n(4.0));
    }

    #[test]
    fn test_equality() {
        let a = Complex { re: n(1.0), im: n(2.0) };
        let b = Complex { re: n(1.0), im: n(2.0) };
        let c = Complex { re: n(1.0), im: n(3.0) };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_addition() {
        let a = Complex { re: n(1.0), im: n(2.0) };
        let b = Complex { re: n(3.0), im: n(4.0) };
        let sum = Complex { re: n(4.0), im: n(6.0) };
        assert_eq!(a + b, sum);
    }

    #[test]
    fn test_multiplication() {
        let a = Complex { re: n(1.0), im: n(2.0) };
        let b = Complex { re: n(3.0), im: n(4.0) };
        // (1 + 2i)(3 + 4i) = (3 - 8) + (4 + 6)i = -5 + 10i
        let product = Complex { re: n(-5.0), im: n(10.0) };
        assert_eq!(a * b, product);
    }

    #[test]
    fn test_conjugate() {
        let a = Complex { re: n(5.0), im: n(-7.0) };
        let conj = Complex { re: n(5.0), im: n(7.0) };
        assert_eq!(a.conjugate(), conj);
    }

    #[test]
    fn test_magnitude_squared() {
        let c = Complex { re: n(3.0), im: n(4.0) };
        assert_eq!(c.norm_sqr(), n(25.0));
    }
}