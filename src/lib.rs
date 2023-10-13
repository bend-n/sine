#![doc = include_str!("../README.md")]
use std::f32::consts::{PI, TAU};
use umath::FF32;
const FRACT_PI_2: f32 = PI / 2.0;

#[inline]
/// Calaculates the cosine of x, approximately. cant say how approximately.
pub fn cos(x: f32) -> f32 {
    let x = unsafe { FF32::new(x) };
    let mut y = x * unsafe { FF32::new(1.0 / TAU) };
    y -= 0.25 + (y + 0.25).floor();
    *(y * 16.0 * (y.abs() - 0.5))
}

#[inline]
/// Calculates the sine of x, approximately. cant say how approximately.
pub fn sin(x: f32) -> f32 {
    cos(x - FRACT_PI_2)
}

/// extension crate for cos/sin.
pub trait Sine {
    /// faster cos
    fn cosf(self) -> Self;
    /// faster sin
    fn sinf(self) -> Self;
}

impl Sine for f32 {
    fn cosf(self) -> Self {
        cos(self)
    }

    fn sinf(self) -> Self {
        sin(self)
    }
}
