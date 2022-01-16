use nalgebra_glm::Vec3;

use crate::misc::camera::Camera;
use crate::misc::rand_gen::RandGenRc;

#[derive(Clone)]
pub struct CameraInfo {
    origin: Vec3,
    center: Vec3,
    up: Vec3,
    v_fov: f32,
    aspect_ratio: f32,
}

impl CameraInfo {
    pub fn new(origin: Vec3, center: Vec3, up: Vec3, v_fov: f32, aspect_ratio: f32) -> Self {
        return Self {
            origin,
            center,
            up,
            v_fov,
            aspect_ratio,
        };
    }
}

impl CameraInfo {
    pub fn build(&self, rand: RandGenRc) -> Camera {
        return Camera::new(
            self.origin,
            self.center,
            self.up,
            self.v_fov,
            self.aspect_ratio,
            rand,
        );
    }
}
