use crate::ray::Ray;
use vec3::Vec3;

pub struct Camera {
    origin: Vec3<f32>,
    horizontal: Vec3<f32>,
    vertical: Vec3<f32>,
    lower_left_corner: Vec3<f32>,
}

impl Camera {
    pub fn new(pos: Vec3<f32>, h: f32, v: f32) -> Self {
        Camera {
            origin: pos,
            horizontal: Vec3::new(h, 0.0, 0.0),
            vertical: Vec3::new(0.0, v, 0.0),
            lower_left_corner: Vec3::new(h / (-2.0), v / (-2.0), -1.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            diraction: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        }
    }
}
