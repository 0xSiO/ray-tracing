use crate::vec3::Vec3;

mod objects;
mod ray;
mod sphere;

pub use objects::Objects;
pub use ray::{Hit, Ray, RayHit};
pub use sphere::Sphere;

/// Point in 3D space: x, y, z
pub type Point = Vec3<f64>;
