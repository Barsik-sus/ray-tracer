use vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub diraction: Vec3<f32>,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3<f32> {
        self.origin + self.diraction * t
    }
}