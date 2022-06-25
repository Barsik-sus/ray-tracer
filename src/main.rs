extern crate image;
use image::RgbImage;
use rand::Rng;
use vec3::Vec3;

mod ray;
use ray::Ray;
mod cam;
use cam::Camera;

#[derive(Debug)]
struct HitRecord {
    t: f32,
    p: Vec3<f32>,
    normal: Vec3<f32>,
}

trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug)]
struct Sphere {
    center: Vec3<f32>,
    radius: f32,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.diraction, &ray.diraction);
        let b = Vec3::dot(&oc, &ray.diraction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let val = |temp: f32| -> Option<HitRecord> {
                if temp < t_max && temp > t_min {
                    let _p = ray.point_at_parameter(temp);
                    return Some(HitRecord {
                        t: temp,
                        p: _p,
                        normal: (_p - self.center) / self.radius,
                    });
                }
                None
            };
            if let Some(value) = val((-b - (discriminant).sqrt()) / a) {
                return Some(value);
            } else if let Some(value) = val((-b + (discriminant).sqrt()) / a) {
                return Some(value);
            }
        }
        None
    }
}

fn random_in_unit_sphere() -> Vec3<f32> {
    let mut p;
    let mut rng = rand::thread_rng();
    while {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
        p.squared_length() >= 1.0
    } {}
    p
}

fn color(ray: Ray, world: &Vec<&dyn Hit>) -> Vec3<f32> {
    if let Some(rec) = {
        let mut rec = None;
        let mut closest_so_far = f32::MAX;
        for obj in world.iter() {
            match obj.hit(&ray, 0.0, closest_so_far) {
                Some(h) => {
                    closest_so_far = h.t;
                    rec = Some(h);
                }
                _ => (),
            }
        }
        rec
    } {
        // return Vec3::new(
        //     rec.normal[0] + rec.p[0] + 1.0,
        //     rec.normal[1] + rec.p[1] + 1.0,
        //     rec.normal[2] + rec.p[2] + 1.0,
        // ) * 0.5;
        return color(
            Ray {
                origin: rec.p,
                diraction: rec.normal + random_in_unit_sphere(),
            },
            world,
        ) * 0.5;
    }
    let u_direct = ray.diraction.as_unit();
    let t = 0.5 * (u_direct[1] + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let width = 640;
    let height = 360;
    let ns = 10;
    let mut img = RgbImage::new(width, height);

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 6.4, 3.6);
    let world = [
        &Sphere {
            center: Vec3::new(0.5, 0.0, -2.0),
            radius: 0.5,
        } as &dyn Hit,
        &Sphere {
            center: Vec3::new(-0.5, 0.0, -1.0),
            radius: 0.5,
        } as &dyn Hit,
        &Sphere {
            center: Vec3::new(0.0, 100.5, -1.0),
            radius: 100.0,
        } as &dyn Hit,
    ];

    let mut rng = rand::thread_rng();
    let start = std::time::Instant::now();

    for y in 0..height {
        for x in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u: f32 = ((x as f32) + rng.gen::<f32>()) / (width as f32);
                let v: f32 = ((y as f32) + rng.gen::<f32>()) / (height as f32);

                let ray = camera.get_ray(u, v);
                col += color(ray, &world.to_vec());
            }

            col /= ns as f32;
            col[0] = col[0].sqrt();
            col[1] = col[1].sqrt();
            col[2] = col[2].sqrt();
            let color = col * 255.99;
            img.put_pixel(x, y, {
                image::Rgb([color[0] as u8, color[1] as u8, color[2] as u8])
            });
        }
    }

    println!("time: {:?}", start.elapsed());
    img.save("img.png").unwrap();
}
