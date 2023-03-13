use crate::{vec3::Vec3, Point};

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
