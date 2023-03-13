use crate::vec3::Vec3;

use super::Point;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pos: Point,
    dir: Vec3<f64>,
}

impl Ray {
    pub fn new(pos: Point, dir: Vec3<f64>) -> Self {
        Self {
            pos,
            dir: dir.normalize(),
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn dir(&self) -> Vec3<f64> {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        self.pos + self.dir * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    /// Point of intersection
    point: Point,
    /// Surface normal at intersection
    normal: Vec3<f64>,
    /// Distance along ray to intersection
    dist: f64,
    /// Whether the ray is incident on the outside of the surface
    outside: bool,
}

impl RayHit {
    pub fn new(ray: Ray, dist: f64, mut normal: Vec3<f64>) -> Self {
        let outside = ray.dir().dot(normal) < 0.0;
        if !outside {
            normal = -normal;
        }

        Self {
            point: ray.at(dist),
            normal,
            dist,
            outside,
        }
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn normal(&self) -> Vec3<f64> {
        self.normal
    }

    pub fn dist(&self) -> f64 {
        self.dist
    }

    pub fn outside(&self) -> bool {
        self.outside
    }
}

pub trait Hit {
    fn find_ray_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<RayHit>;
}
