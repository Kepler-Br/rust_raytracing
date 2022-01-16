use nalgebra_glm::Vec3;

use crate::hittables::hittable::Hittable;
use crate::hittables::sphere::Sphere;
use crate::materials::material::MaterialRc;
use crate::scene::hittable_info::HittableInfo;

#[derive(Clone)]
pub struct SphereInfo {
    name: String,
    material_name: String,
    center: Vec3,
    radius: f32,
}

impl SphereInfo {
    pub fn new(name: &str, material_name: &str, center: Vec3, radius: f32) -> Self {
        return Self {
            name: name.to_string(),
            material_name: material_name.to_string(),
            center,
            radius,
        };
    }

    pub fn boxed(name: &str, material_name: &str, center: Vec3, radius: f32) -> Box<Self> {
        return Box::new(Self::new(name, material_name, center, radius));
    }
}

impl HittableInfo for SphereInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_material_name(&self) -> &str {
        return &self.material_name;
    }

    fn build(&self, material: MaterialRc) -> Box<dyn Hittable> {
        return Sphere::boxed(self.center, self.radius, material);
    }
}
