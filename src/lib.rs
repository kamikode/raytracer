//! A simple software raytracer based on the book "The Ray Tracer Challenge".
#![warn(missing_debug_implementations)]

mod canvas;
mod intersection;
mod material;
mod objects;
mod point_light;
mod primitives;
mod ray;

pub use canvas::Canvas;
pub use intersection::Intersection;
pub use material::Material;
pub use objects::sphere::Sphere;
pub use point_light::PointLight;
pub use primitives::color::Color;
pub use primitives::float::Float;
pub use primitives::matrix::{Invertible, Matrix, Matrix2x2, Matrix3x3, Matrix4x4};
pub use primitives::tuple::{Point, Vector};
pub use ray::{get_hit, Ray};
