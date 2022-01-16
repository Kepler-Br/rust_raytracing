use nalgebra_glm::Vec3;

use crate::materials::emission::Emission;
use crate::materials::material::MaterialRc;
use crate::misc::rand_gen::RandGenRc;
use crate::scene::material_info::MaterialInfo;

#[derive(Clone)]
pub struct EmissionInfo {
    name: String,
    color: Vec3,
    power: f32,
}

impl EmissionInfo {
    pub fn new(name: &str, color: Vec3, power: f32) -> Self {
        return Self {
            name: name.to_string(),
            color,
            power,
        };
    }

    pub fn boxed(name: &str, color: Vec3, power: f32) -> Box<Self> {
        return Box::new(Self::new(name, color, power));
    }
}

impl MaterialInfo for EmissionInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn build(&self, rand: RandGenRc) -> MaterialRc {
        return Emission::new_rc(self.color, self.power);
    }
}
