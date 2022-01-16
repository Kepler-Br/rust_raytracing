use nalgebra_glm::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        return Ray {
            origin,
            direction: direction.normalize(),
        };
    }

    /// New Ray without direction normalization
    pub fn no_norm(origin: Vec3, direction: Vec3) -> Ray {
        return Ray { origin, direction };
    }

    pub fn default() -> Ray {
        return Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 0.0),
        };
    }
}

impl Ray {
    pub fn get_origin(&self) -> &Vec3 {
        return &self.origin;
    }

    pub fn get_direction(&self) -> &Vec3 {
        return &self.direction;
    }

    pub fn get_at(&self, length: f32) -> Vec3 {
        return self.origin + self.direction * length;
    }
}
