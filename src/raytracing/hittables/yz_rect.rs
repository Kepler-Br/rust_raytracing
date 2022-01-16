use nalgebra_glm::{Vec2, Vec3};

use crate::hittables::bounding::aabb::Aabb;
use crate::hittables::hittable::Hittable;
use crate::materials::material::MaterialRc;
use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;

pub struct YzRect {
    point_one: Vec2,
    point_two: Vec2,
    normal_displacement: f32,
    material: MaterialRc,
}

impl YzRect {
    pub fn new(
        point_one: Vec2,
        point_two: Vec2,
        normal_displacement: f32,
        material: MaterialRc,
    ) -> Self {
        return YzRect {
            point_one,
            point_two,
            normal_displacement,
            material,
        };
    }

    pub fn boxed(
        point_one: Vec2,
        point_two: Vec2,
        normal_displacement: f32,
        material: MaterialRc,
    ) -> Box<Self> {
        return Box::new(YzRect {
            point_one,
            point_two,
            normal_displacement,
            material,
        });
    }

    pub fn square_boxed(position: Vec3, edge_length: f32, material: MaterialRc) -> Box<Self> {
        let half_edge = edge_length / 2.0;
        let point_one = Vec2::new(-half_edge + position.y, -half_edge + position.z);
        let point_two = Vec2::new(half_edge + position.y, half_edge + position.z);

        return Self::boxed(point_one, point_two, position.x, material);
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let t = (self.normal_displacement - ray.get_origin().x) / ray.get_direction().x;

        if t < t_min || t > t_max {
            return false;
        }

        let y = ray.get_origin().y + t * ray.get_direction().y;
        let z = ray.get_origin().z + t * ray.get_direction().z;

        if y < self.point_one.x
            || y > self.point_two.x
            || z < self.point_one.y
            || z > self.point_two.y
        {
            return false;
        }

        const SELF_NORMAL: Vec3 = Vec3::new(1.0, 0.0, 0.0);

        if ray.get_direction().dot(&SELF_NORMAL) > 0.0 {
            record.set_normal(SELF_NORMAL);
            record.set_is_front_face(true);
        } else {
            record.set_normal(-SELF_NORMAL);
            record.set_is_front_face(false);
        };

        record.set_distance(t);
        record.set_material(self.material.clone());
        record.set_point(ray.get_at(t));

        return true;
    }

    fn bounding_box(&self) -> Option<Aabb> {
        return Option::Some(Aabb::new(
            Vec3::new(
                self.normal_displacement,
                f32::min(self.point_one.x, self.point_two.x),
                f32::min(self.point_one.y, self.point_two.y),
            ),
            Vec3::new(
                self.normal_displacement,
                f32::max(self.point_one.x, self.point_two.x),
                f32::max(self.point_one.y, self.point_two.y),
            ),
        ));
    }
}
