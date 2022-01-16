use nalgebra_glm::Vec3;

use crate::materials::material::MaterialRc;
use crate::materials::reflective::Reflective;
use crate::misc::rand_gen::RandGenRc;
use crate::scene::material_info::MaterialInfo;

#[derive(Clone)]
pub struct ReflectiveInfo {
    name: String,
    color: Vec3,
    reflectiveness: f32,
}

impl ReflectiveInfo {
    pub fn new(name: &str, color: Vec3, reflectiveness: f32) -> Self {
        return Self {
            name: name.to_string(),
            color,
            reflectiveness,
        };
    }

    pub fn boxed(name: &str, color: Vec3, reflectiveness: f32) -> Box<Self> {
        return Box::new(Self::new(name, color, reflectiveness));
    }
}

impl MaterialInfo for ReflectiveInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn build(&self, rand: RandGenRc) -> MaterialRc {
        return Reflective::new_rc(self.color, self.reflectiveness, rand);
    }
}
