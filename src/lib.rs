//! A simple software raytracer based on the book "The Ray Tracer Challenge".
#![warn(missing_debug_implementations)]

mod canvas;
mod intersection;
mod objects;
mod primitives;
mod ray;

pub use canvas::Canvas;
pub use intersection::Intersection;
pub use objects::sphere::Sphere;
pub use primitives::color::Color;
pub use primitives::matrix::{Invertible, Matrix, Matrix2x2, Matrix3x3, Matrix4x4};
pub use primitives::tuple::{Point, Vector};
pub use ray::Ray;
