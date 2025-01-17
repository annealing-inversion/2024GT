pub use crate::vec3::Vec3;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir , tm: 0.0}
    }
    pub fn new_with_time(orig: Vec3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn at(&self, t: f64) -> Vec3 {
        let res = Vec3::new(self.dir.x * t, self.dir.y * t, self.dir.z * t);
        res + self.orig
    }

}