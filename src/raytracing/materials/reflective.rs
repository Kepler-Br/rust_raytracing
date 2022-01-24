use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm::{reflect_vec, Vec3};

use crate::materials::material::{Material, MaterialRc};
use crate::misc::hit_record::HitRecord;
use crate::misc::rand_gen::RandGenRc;
use crate::misc::ray::Ray;

pub struct Reflective {
    albedo: Vec3,
    reflectiveness: f32,
    rand_generator: RandGenRc,
}

impl Reflective {
    pub fn new(albedo: Vec3, reflectiveness: f32, rand_generator: RandGenRc) -> Self {
        return Self {
            albedo,
            reflectiveness,
            rand_generator,
        };
    }

    pub fn new_rc(albedo: Vec3, reflectiveness: f32, rand_generator: RandGenRc) -> MaterialRc {
        return Rc::new(RefCell::new(Box::new(Self::new(
            albedo,
            reflectiveness,
            rand_generator,
        ))));
    }
}

impl Material for Reflective {
    fn scatter(
        &mut self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let corrected_normal = if Vec3::dot(ray.get_direction(), hit_record.get_normal()) > 0.0 {
            -*hit_record.get_normal()
        } else {
            *hit_record.get_normal()
        };

        let reflection = reflect_vec(ray.get_direction(), &corrected_normal);
        let target = RefCell::borrow_mut(&self.rand_generator).unit_hemisphere(&corrected_normal);

        let scatter_direction = (target + reflection * self.reflectiveness).normalize();
        *scattered = Ray::new(*hit_record.get_point(), scatter_direction);
        *attenuation = self.get_attenuation();

        return true;
    }

    fn get_attenuation(&self) -> Vec3 {
        return self.albedo;
    }
}
