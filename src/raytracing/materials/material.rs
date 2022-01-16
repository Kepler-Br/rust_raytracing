use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm::Vec3;

use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;

pub type MaterialRc = Rc<RefCell<Box<dyn Material>>>;

pub trait Material {
    fn scatter(
        &mut self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn get_attenuation(&self) -> Vec3;
}
