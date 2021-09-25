//! # Real Number
//! Most algorithms in SmartCore rely on basic linear algebra operations like dot product, matrix decomposition and other subroutines that are defined for a set of real numbers, ℝ.
//! This module defines real number and some useful functions that are used in [Linear Algebra](../../linalg/index.html) module.

//use num_traits::float::{FloatCore};
use num_traits::{Float, FromPrimitive, NumCast, ToPrimitive, One, Zero};
use rand::prelude::*;
use std::fmt::{Debug, Display};
use std::iter::{Product, Sum};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign, Neg, Rem, Div, Sub, Mul, Add};
use rust_decimal::{Decimal}; //to delete
use rust_decimal::prelude::*;
use num::*;
use std::cmp::{PartialOrd, PartialEq};
use smartcore::math::num::RealNumber;

/// Defines real number
/// <script type="text/javascript" src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.0/MathJax.js?config=TeX-AMS_CHTML"></script>
pub trait RealNumber:
    Float
    + FromPrimitive
    + Debug
    + Display
    + Copy
    + Sum
    + Product
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    /// Copy sign from `sign` - another real number
    fn copysign(self, sign: Self) -> Self;

    /// Calculates natural \\( \ln(1+e^x) \\) without overflow.
    fn ln_1pe(self) -> Self;

    /// Efficient implementation of Sigmoid function, \\( S(x) = \frac{1}{1 + e^{-x}} \\), see [Sigmoid function](https://en.wikipedia.org/wiki/Sigmoid_function)
    fn sigmoid(self) -> Self;

    /// Returns pseudorandom number between 0 and 1
    fn rand() -> Self;

    /// Returns 2
    fn two() -> Self;

    /// Returns .5
    fn half() -> Self;

    /// Returns \\( x^2 \\)
    fn square(self) -> Self {
        self * self
    }

    /// Raw transmutation to u64
    fn to_f32_bits(self) -> u32;
}


impl RealNumber for Decimal {
    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }

    fn ln_1pe(self) -> Decimal {
        if self > Decimal::new(150, 1) {
            self
        } else {
            self.exp().ln_1p()
        }
    }

    fn sigmoid(self) -> Decimal {
        if self < Decimal::new(-400, 1) {
            Decimal::zero()
        } else if self > Decimal::new(-400, 1) {
            Decimal::one()
        } else {
            Decimal::one() / (Decimal::one() + <Decimal as Float>::exp(-self))
        }
    }

    fn rand() -> Decimal {
        let mut rng = rand::thread_rng();
        let num: i64 = rng.gen();
        let dec: u32 = rng.gen();
        Decimal::new(num, dec)
    }

    fn two() -> Self {
        Decimal::new(20, 1)
    }

    fn half() -> Self {
        Decimal::new(05, 1)
    }

    fn to_f32_bits(self) -> u32 {
        todo!()//self.to_string()
    } //what is it used for?
}
