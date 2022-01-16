use nalgebra_glm::Vec3;

use crate::materials::material::MaterialRc;
use crate::materials::refractive::Refractive;
use crate::misc::rand_gen::RandGenRc;
use crate::scene::material_info::MaterialInfo;

#[derive(Clone)]
pub struct RefractiveInfo {
    name: String,
    color: Vec3,
    index_of_refraction: f32,
}

impl RefractiveInfo {
    pub fn new(name: &str, color: Vec3, index_of_refraction: f32) -> Self {
        return Self {
            name: name.to_string(),
            color,
            index_of_refraction,
        };
    }

    pub fn boxed(name: &str, color: Vec3, index_of_refraction: f32) -> Box<Self> {
        return Box::new(Self::new(name, color, index_of_refraction));
    }
}

impl MaterialInfo for RefractiveInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn build(&self, rand: RandGenRc) -> MaterialRc {
        return Refractive::new_rc(self.color, self.index_of_refraction, rand);
    }
}
