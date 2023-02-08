use crate::{color::Color, hit::HitRecord, ray::Ray, vec::Vec3D};

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3D::random_in_unit_sphere().normalize();
        if scatter_direction.is_near_zero() {
            // Catch degenerate scatter direction
            scatter_direction = hit_record.normal;
        }
        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);

        Some((self.albedo, scattered_ray))
    }
}

pub struct Hemisphere {
    albedo: Color,
}

impl Hemisphere {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Scatter for Hemisphere {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction =
            hit_record.hit_point + Vec3D::random_in_hemisphere(hit_record.normal);
        let scattered_ray = Ray::new(
            hit_record.hit_point,
            scatter_direction - hit_record.hit_point,
        );

        Some((self.albedo, scattered_ray))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz,
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_vector = ray_in.direction.reflect(hit_record.normal).normalize();
        let scattered_ray = Ray::new(
            hit_record.hit_point,
            reflected_vector + self.fuzz * Vec3D::random_in_unit_sphere(),
        );

        if scattered_ray.direction.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }
}