use num_traits::Num;

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray<T: Num + Copy> {
    pub pos: Vec3<T>,
    pub dir: Vec3<T>,
}

impl<T: Num + Copy> Ray<T> {
    pub fn at(&self, t: T) -> Vec3<T> {
        self.pos + self.dir * t
    }
}
