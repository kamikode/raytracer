//! A simple software raytracer based on the book "The Ray Tracer Challenge".

#![warn(missing_debug_implementations)]

mod canvas;
mod primitives;

pub use canvas::Canvas;
pub use primitives::color::Color;
pub use primitives::matrix::Matrix;
pub use primitives::tuple::{Point, Vector};
