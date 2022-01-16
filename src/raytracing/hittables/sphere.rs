use nalgebra_glm::Vec3;

use crate::hittables::bounding::aabb::Aabb;
use crate::hittables::hittable::Hittable;
use crate::materials::material::MaterialRc;
use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: MaterialRc,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialRc) -> Self {
        return Sphere {
            center,
            radius,
            material,
        };
    }

    pub fn boxed(center: Vec3, radius: f32, material: MaterialRc) -> Box<Self> {
        return Box::new(Self::new(center, radius, material));
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.get_origin() - self.center;
        let a = ray.get_direction().magnitude().powf(2.0);
        let half_b = oc.dot(ray.get_direction());
        let c = oc.magnitude().powf(2.0) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;

            if root < t_min || root > t_max {
                return false;
            }
        }

        record.set_distance(root);
        record.set_point(ray.get_at(record.get_distance()));
        record.set_normal(((record.get_point() - self.center) / self.radius).normalize());
        record.set_material(self.material.clone());

        return true;
    }

    fn bounding_box(&self) -> Option<Aabb> {
        return Option::Some(Aabb::new(
            self.center.add_scalar(-self.radius),
            self.center.add_scalar(self.radius),
        ));
    }
}
