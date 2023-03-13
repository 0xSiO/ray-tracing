use super::{Hit, Ray, RayHit};

pub struct Objects(Vec<Box<dyn Hit>>);

impl Objects {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, object: impl Hit + 'static) {
        self.0.push(Box::new(object))
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}

impl Hit for Objects {
    fn find_ray_hit(&self, ray: Ray, min_dist: f64, max_dist: f64) -> Option<RayHit> {
        let mut closest_hit: Option<RayHit> = None;

        for object in &self.0 {
            let closest_dist = closest_hit.map_or(max_dist, |hit| hit.dist());
            if let Some(hit) = object.find_ray_hit(ray, min_dist, closest_dist) {
                closest_hit.replace(hit);
            }
        }

        closest_hit
    }
}
