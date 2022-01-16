use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm::Vec3;

use crate::materials::material::{Material, MaterialRc};
use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;

pub struct Emission {
    color: Vec3,
    power: f32,
}

impl Emission {
    pub fn new(color: Vec3, power: f32) -> Self {
        return Emission { color, power };
    }

    pub fn new_rc(color: Vec3, power: f32) -> MaterialRc {
        return Rc::new(RefCell::new(Box::new(Emission::new(color, power))));
    }
}

impl Material for Emission {
    fn scatter(
        &mut self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.get_attenuation();

        return false;
    }

    fn get_attenuation(&self) -> Vec3 {
        return self.color * self.power;
    }
}
