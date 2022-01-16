use nalgebra_glm::Vec3;

use crate::hittables::hittable::Hittable;
use crate::hittables::xy_rect::XyRect;
use crate::materials::material::MaterialRc;
use crate::scene::hittable_info::HittableInfo;

#[derive(Clone)]
pub struct XyRectInfo {
    name: String,
    material_name: String,
    position: Vec3,
    edge_length: f32,
}

impl XyRectInfo {
    pub fn square_new(name: &str, material_name: &str, position: Vec3, edge_length: f32) -> Self {
        return Self {
            name: name.to_string(),
            material_name: material_name.to_string(),
            position,
            edge_length,
        };
    }

    pub fn square_boxed(name: &str, material_name: &str, position: Vec3, edge_length: f32) -> Box<Self> {
        return Box::new(Self::square_new(
            name,
            material_name,
            position,
            edge_length,
        ));
    }
}

impl HittableInfo for XyRectInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_material_name(&self) -> &str {
        return &self.material_name;
    }

    fn build(&self, material: MaterialRc) -> Box<dyn Hittable> {
        return XyRect::square_boxed(self.position, self.edge_length, material);
    }
}
