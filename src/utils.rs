use std::ops::{Add, Div, Mul, Sub};

/// Takes [`x`] which falls inclusivly between [`a`] and [`b`] to [`y`] which falls between [`c`] and [`d`]
pub fn map<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy>(
    x: T,
    a: T,
    b: T,
    c: T,
    d: T,
) -> T {
    (x - a) / (b - a) * (d - c) + c
}
