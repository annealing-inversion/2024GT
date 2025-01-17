use crate::raytracer::*;
use crate::vec3::Vec3;
use crate::perlin::perlin;
use std::rc::Rc;
use std::sync::Arc;

pub trait texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct solid_color {
    albedo: Vec3,
}

impl solid_color {
    pub fn new(c: Vec3) -> Self {
        Self { albedo: c }
    }
    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { albedo: Vec3::new(r, g, b) }
    }
}

impl texture for solid_color {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct checker_texture {
    inv_scale: f64,
    even: Arc<dyn texture + Send + Sync>,
    odd: Arc<dyn texture + Send + Sync>,
}

impl checker_texture {
    pub fn new(scale: f64, even: Arc<dyn texture + Send + Sync>, odd: Arc<dyn texture + Send + Sync>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd, 
        }
    }
    pub fn new_from_colors(scale: f64, c1: Vec3, c2: Vec3) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(solid_color::new(c1)),
            odd: Arc::new(solid_color::new(c2)),
            // odd: Arc::new(solid_color::new(c1)),
            // even: Arc::new(solid_color::new(c2)),
        }
    }
}

impl texture for checker_texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let xinteger = (self.inv_scale * p.x).floor() as i32;
        let yinteger = (self.inv_scale * p.y).floor() as i32;
        let zinteger = (self.inv_scale * p.z).floor() as i32;

        let iseven = (xinteger + yinteger + zinteger) % 2 == 0;
        if iseven {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct image_texture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    bytes_per_scanline: usize,
}

impl image_texture {
    pub fn new(filename: &str) -> Self {

        let bytes_per_pixel = 3;
        let img = image::open(filename).unwrap().to_rgb8();

        // println!("Image loaded: {}x{}", img.width(), img.height());
        let width = img.width();
        let height = img.height();
        let bytes_per_scanline = (bytes_per_pixel * width) as usize; 
        
        Self {
            data: img.into_raw(),
            width,
            height,
            bytes_per_pixel,
            bytes_per_scanline,
        }

    }
}
impl texture for image_texture {
    fn value (&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if self.data.is_empty() {
            return Vec3::new(0.0, 1.0, 1.0);
        }
        // println!("u: {}, v: {}", u, v);
        let u2 = u;
        let v2 = 1.0 - v;
        let mut i = (u2 * self.width as f64) as usize;
        let mut j = (v2 * self.height as f64) as usize;
        if i >= self.width as usize {
            i = self.width as usize - 1;
        }
        if j >= self.height as usize {
            j = self.height as usize - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel_index = self.bytes_per_pixel as usize * i + self.bytes_per_scanline * j;
        // println!("i: {}, j: {}, pixel_index: {}", i, j, pixel_index);
        let r = self.data[pixel_index] as f64 * color_scale;
        let g = self.data[pixel_index + 1] as f64 * color_scale;
        let b = self.data[pixel_index + 2] as f64 * color_scale;
        // println!("r: {}, g: {}, b: {}", r, g, b);
        Vec3::new(r, g, b)
    }
}

pub struct noise_texture {
    noise: perlin,
    scale: f64,
}

impl noise_texture {
    pub fn new() -> Self {
        Self {
            noise: perlin::new(), 
            scale: 1.0,
        }
    }
    pub fn new_with_scale(s: f64) -> Self {
        Self {
            noise: perlin::new(),
            scale: s,
        }
    }   
}

impl texture for noise_texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        // Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(&(*p * self.scale))
        // Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(*p * self.scale)))
        // Vec3::new(1.0, 1.0, 1.0) * self.noise.turb(p, 7)
        Vec3::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}