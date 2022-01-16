use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm::Vec3;

use crate::materials::material::{Material, MaterialRc};
use crate::misc::hit_record::HitRecord;
use crate::misc::rand_gen::RandGenRc;
use crate::misc::ray::Ray;

pub struct Lambertian {
    albedo: Vec3,
    rand_generator: RandGenRc,
}

impl Lambertian {
    pub fn new(albedo: Vec3, rand_generator: RandGenRc) -> Self {
        return Lambertian {
            albedo,
            rand_generator,
        };
    }

    pub fn new_rc(albedo: Vec3, rand_generator: RandGenRc) -> MaterialRc {
        return Rc::new(RefCell::new(Box::new(Self::new(albedo, rand_generator))));
    }
}

impl Material for Lambertian {
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

        let target = RefCell::borrow_mut(&self.rand_generator).unit_hemisphere(&corrected_normal);

        // let target;
        //
        // if unit_sphere_sample.dot(hit_record.get_normal()) > 0.0 {
        //     target = -unit_sphere_sample;
        // } else {
        //     target = unit_sphere_sample;
        // }

        let scatter_direction = (target).normalize();
        *scattered = Ray::new(*hit_record.get_point(), scatter_direction);
        *attenuation = self.get_attenuation();
        // *attenuation = hit_record.get_normal();

        return true;

        // auto scatter_direction = rec.normal + random_unit_vector();
        // scattered = ray(rec.p, scatter_direction);
        // attenuation = albedo;
        // return true;
    }

    fn get_attenuation(&self) -> Vec3 {
        return self.albedo;
    }
}
