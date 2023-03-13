use super::{Hit, Point, Ray, RayHit};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hit for Sphere {
    // Find point at which ray hits sphere, using this equation:
    // (t**2)(d⋅d) + 2td⋅(P−C) + (P−C)⋅(P−C) - r**2 = 0
    //
    // P: position of the ray
    // d: direction of the ray
    // C: center of the sphere
    // r: radius of the sphere
    fn find_ray_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<RayHit> {
        let dir = ray.dir();
        let Sphere { center, radius } = *self;

        let a = dir.dot(dir);
        // Quadratic formula optimization where b = 2h, h = d⋅(P−C)
        let h = dir.dot(ray.pos() - center);
        let c = (ray.pos() - center).dot(ray.pos() - center) - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        // Find nearest root within given range
        let mut root = (-h - discriminant.sqrt()) / a;
        if root < min_dist || root > max_dist {
            root = (-h + discriminant.sqrt()) / a;
            if root < min_dist || root > max_dist {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - center).normalize();

        Some(RayHit::new(ray, root, normal))
    }
}
