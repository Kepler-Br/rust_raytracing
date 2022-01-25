use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm::{reflect_vec, Vec3};

use crate::materials::material::{Material, MaterialRc};
use crate::misc::hit_record::HitRecord;
use crate::misc::rand_gen::RandGenRc;
use crate::misc::ray::Ray;

pub struct Refractive {
    albedo: Vec3,
    index_of_refraction: f32,
    rand_generator: RandGenRc,
}

impl Refractive {
    pub fn new(albedo: Vec3, index_of_refraction: f32, rand_generator: RandGenRc) -> Self {
        return Self {
            albedo,
            index_of_refraction,
            rand_generator,
        };
    }

    pub fn new_rc(albedo: Vec3, index_of_refraction: f32, rand_generator: RandGenRc) -> MaterialRc {
        return Rc::new(RefCell::new(Box::new(Self::new(
            albedo,
            index_of_refraction,
            rand_generator,
        ))));
    }
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(Vec3::dot(&-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.magnitude_squared())) * n;

    return r_out_perp + r_out_parallel;
}

impl Material for Refractive {
    fn scatter(
        &mut self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        // attenuation = color(1.0, 1.0, 1.0);
        // double refraction_ratio = rec.front_face ? (1.0/ir) : ir;
        //
        // vec3 unit_direction = unit_vector(r_in.direction());
        // vec3 refracted = refract(unit_direction, rec.normal, refraction_ratio);
        //
        // scattered = ray(rec.p, refracted);

        let refraction_ratio = if !hit_record.get_is_front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.get_direction();
        let refracted = refract(unit_direction, hit_record.get_normal(), refraction_ratio);

        *attenuation = self.albedo;

        if RefCell::borrow_mut(&self.rand_generator).uniform() > 0.85 {
            *scattered = Ray::new(
                *hit_record.get_point(),
                reflect_vec(ray.get_direction(), hit_record.get_normal()),
            );
        } else {
            *scattered = Ray::new(*hit_record.get_point(), refracted);
        }

        // *scattered = Ray::new(*hit_record.get_point(), *ray.get_direction());

        // let corrected_normal = if Vec3::dot(ray.get_direction(), hit_record.get_normal()) > 0.0 {
        //     *hit_record.get_normal()
        // } else {
        //     -*hit_record.get_normal()
        // };
        //
        // let reflection = reflect_vec(ray.get_direction(), &corrected_normal);
        // let target = RefCell::borrow_mut(&self.rand_generator).unit_hemisphere(&corrected_normal);
        //
        // let scatter_direction = (target + reflection * self.reflectiveness).normalize();
        // *scattered = Ray::new(*hit_record.get_point(), scatter_direction);
        // *attenuation = self.get_attenuation();

        return true;
    }

    fn get_attenuation(&self) -> Vec3 {
        return self.albedo;
    }
}
