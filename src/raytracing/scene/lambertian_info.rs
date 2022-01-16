use nalgebra_glm::Vec3;

use crate::materials::lambertian::Lambertian;
use crate::materials::material::MaterialRc;
use crate::misc::rand_gen::RandGenRc;
use crate::scene::material_info::MaterialInfo;

#[derive(Clone)]
pub struct LambertianInfo {
    name: String,
    color: Vec3,
}

impl LambertianInfo {
    pub fn new(name: &str, color: Vec3) -> Self {
        return Self {
            name: name.to_string(),
            color,
        };
    }

    pub fn boxed(name: &str, color: Vec3) -> Box<Self> {
        return Box::new(Self::new(name, color));
    }
}

impl MaterialInfo for LambertianInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn build(&self, rand: RandGenRc) -> MaterialRc {
        return Lambertian::new_rc(self.color, rand);
    }
}
