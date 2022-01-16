use std::mem::swap;

use nalgebra_glm::Vec3;

use crate::misc::ray::Ray;

pub struct Aabb {
    minimum: Vec3,
    maximum: Vec3,
}

impl Aabb {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        return Self { minimum, maximum };
    }
}

impl Aabb {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let inv_direction = 1.0 / ray.get_direction()[i];
            let mut t0 = (self.minimum[i] - ray.get_origin()[i]) * inv_direction;
            let mut t1 = (self.maximum[i] - ray.get_origin()[i]) * inv_direction;

            if inv_direction < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 > t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }

    pub fn surrounding(&self, other: Self) -> Self {
        let min = Vec3::new(
            f32::min(self.minimum.x, other.minimum.x),
            f32::min(self.minimum.y, other.minimum.y),
            f32::min(self.minimum.z, other.minimum.z),
        );
        let max = Vec3::new(
            f32::max(self.maximum.x, other.maximum.x),
            f32::max(self.maximum.y, other.maximum.y),
            f32::max(self.maximum.z, other.maximum.z),
        );

        return Self::new(min, max);
    }
}
