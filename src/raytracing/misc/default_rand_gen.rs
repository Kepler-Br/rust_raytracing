use std::cell::RefCell;
use std::rc::Rc;

use nalgebra_glm::{Vec2, Vec3};
use rand::{Rng, thread_rng};

use crate::misc::rand_gen::{RandGen, RandGenRc};

pub struct DefaultRandGen {}

impl Default for DefaultRandGen {
    fn default() -> Self {
        return DefaultRandGen {};
    }
}

impl DefaultRandGen {
    pub fn new_rc() -> RandGenRc {
        return Rc::new(RefCell::new(Box::new(DefaultRandGen::default())));
    }
}

impl RandGen for DefaultRandGen {
    fn unit_sphere(&mut self) -> Vec3 {
        let mut rng = thread_rng();

        loop {
            let vec = Vec3::from_fn(|_, _| rng.gen::<f32>() * 2.0 - 1.0);

            if vec.magnitude() < 1.0 {
                return vec;
            }
        }
    }

    fn unit_hemisphere(&mut self, normal: &Vec3) -> Vec3 {
        let vec = self.unit_sphere();

        return if Vec3::dot(&vec, normal) > 0.0 {
            vec
        } else {
            -vec
        };
    }

    fn unit_disk(&mut self) -> Vec2 {
        let mut rng = thread_rng();

        loop {
            // let vec = Vec2::from_fn(|_, _| rng.gen::<f32>() * 2.0 - 1.0);
            let vec = Vec2::new(rng.gen::<f32>() * 2.0 - 1.0, rng.gen::<f32>() * 2.0 - 1.0);

            if vec.magnitude() < 1.0 {
                return vec;
            }
        }
    }

    fn uniform(&mut self) -> f32 {
        return thread_rng().gen();
    }
}
