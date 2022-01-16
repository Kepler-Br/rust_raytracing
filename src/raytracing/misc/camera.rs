use nalgebra_glm::{look_at, Vec3};

use crate::misc::rand_gen::RandGenRc;
use crate::misc::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    rand: RandGenRc,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        center: Vec3,
        up: Vec3,
        v_fov: f32,
        aspect_ratio: f32,
        rand: RandGenRc,
    ) -> Self {
        let theta = v_fov.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let mat = look_at(&origin, &center, &up);

        let v = Vec3::new(mat.row(0)[0], mat.row(0)[1], mat.row(0)[2]);
        let u = Vec3::new(mat.row(1)[0], mat.row(1)[1], mat.row(1)[2]);
        let w = Vec3::new(mat.row(2)[0], mat.row(2)[1], mat.row(2)[2]);

        let focus_dist = 1.0;
        let aperture = 0.01;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            rand,
        };
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * self.rand.borrow_mut().unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        );
    }
}
