
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::{hit_record, Hittable};
use std::rc::Rc;
use std::vec::Vec;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let mut temp_rec = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        };
        // bool hit_anything = false;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            // if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // rec = temp_rec;

                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;

            }
        }
        return hit_anything;
    }
}