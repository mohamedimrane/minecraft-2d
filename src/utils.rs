use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use bevy::prelude::Vec2;

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

pub fn leans_to_left(a: f32) -> bool {
    a > 3. * PI / 2.
}
pub fn leans_to_right(a: f32) -> bool {
    a < 3. * PI / 2.
}

pub fn in_reach(tr1: Vec2, tr2: Vec2, reach: f32, block_size: f32) -> bool {
    tr1.x < tr2.x + reach * block_size
        && tr1.x > tr2.x - reach * block_size
        && tr1.y < tr2.y + reach * block_size
        && tr1.y > tr2.y - reach * block_size
}
