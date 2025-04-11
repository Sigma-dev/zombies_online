use bevy::prelude::*;
use std::{f32::consts::TAU, ops::Range};

use rand::Rng;

pub fn random_point_in_donut(r_inner: f32, r_outer: f32) -> Vec2 {
    let mut rng = rand::rng();

    let theta = rng.random_range(0.0..TAU);

    let r_squared = rng.random_range(r_inner.powi(2)..r_outer.powi(2));
    let r = r_squared.sqrt();

    Vec2::new(r * theta.cos(), r * theta.sin())
}

pub fn random_float(range: Range<f32>) -> f32 {
    let mut rng = rand::rng();

    return rng.random_range(range);
}
