use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::aabb;
use std::rc::Rc;


pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat:Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}
impl hit_record {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal * -1.0};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool;
    fn bounding_box(&self) -> aabb;
}