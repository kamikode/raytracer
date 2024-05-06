//! A simple software raytracer based on the book "The Ray Tracer Challenge".

#![warn(missing_debug_implementations)]

mod canvas;
mod types;

pub use canvas::Canvas;
pub use types::color::Color;
pub use types::tuple::{Point, Vector};
