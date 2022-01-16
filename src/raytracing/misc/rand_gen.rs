use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm::{Vec2, Vec3};

pub type RandGenRc = Rc<RefCell<Box<dyn RandGen>>>;

pub trait RandGen {
    fn unit_sphere(&mut self) -> Vec3;
    fn unit_hemisphere(&mut self, normal: &Vec3) -> Vec3;
    fn unit_disk(&mut self) -> Vec2;
    fn uniform(&mut self) -> f32;
}
