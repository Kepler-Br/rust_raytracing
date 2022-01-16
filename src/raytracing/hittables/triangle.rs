use nalgebra_glm::Vec3;

use crate::hittables::bounding::aabb::Aabb;
use crate::hittables::hittable::Hittable;
use crate::materials::material::MaterialRc;
use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;

pub struct Triangle {
    point_one: Vec3,
    point_two: Vec3,
    point_three: Vec3,
    material: MaterialRc,
}

impl Triangle {
    pub fn new(point_one: Vec3, point_two: Vec3, point_three: Vec3, material: MaterialRc) -> Self {
        return Self {
            point_one,
            point_two,
            point_three,
            material,
        };
    }

    pub fn boxed(
        point_one: Vec3,
        point_two: Vec3,
        point_three: Vec3,
        material: MaterialRc,
    ) -> Box<Self> {
        return Box::new(Self::new(point_one, point_two, point_three, material));
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let v0 = self.point_one;
        let v1 = self.point_two;
        let v2 = self.point_three;
        let ro = ray.get_origin();
        let rd = ray.get_direction();

        let v1v0 = v1 - v0;
        let v2v0 = v2 - v0;
        let rov0 = ro - v0;
        let n = Vec3::cross(&v1v0, &v2v0);
        let q = Vec3::cross(&rov0, &rd);
        let d = 1.0 / Vec3::dot(&rd, &n);
        let u = d * Vec3::dot(&-q, &v2v0);
        let v = d * Vec3::dot(&q, &v1v0);

        if u < 0.0 || u > 1.0 || v < 0.0 || (u + v) > 1.0 {
            return false;
        }

        let t = d * Vec3::dot(&-n, &rov0);

        if t < t_min || t > t_max {
            return false;
        }

        record.set_normal(n);
        record.set_distance(t);
        record.set_point(ray.get_at(t));
        record.set_material(self.material.clone());

        return true;
    }

    fn bounding_box(&self) -> Option<Aabb> {
        return Option::Some(Aabb::new(
            Vec3::new(
                f32::min(f32::min(self.point_one.x, self.point_two.x), self.point_three.x),
                f32::min(f32::min(self.point_one.y, self.point_two.y), self.point_three.y),
                f32::min(f32::min(self.point_one.z, self.point_two.z), self.point_three.z),
            ),
            Vec3::new(
                f32::max(f32::max(self.point_one.x, self.point_two.x), self.point_three.x),
                f32::max(f32::max(self.point_one.y, self.point_two.y), self.point_three.y),
                f32::max(f32::max(self.point_one.z, self.point_two.z), self.point_three.z),
            ),
        ));
    }
}
