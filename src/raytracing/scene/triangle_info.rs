use nalgebra_glm::Vec3;

use crate::hittables::hittable::Hittable;
use crate::hittables::triangle::Triangle;
use crate::materials::material::MaterialRc;
use crate::scene::hittable_info::HittableInfo;

#[derive(Clone)]
pub struct TriangleInfo {
    name: String,
    material_name: String,
    point_one: Vec3,
    point_two: Vec3,
    point_three: Vec3,
}

impl TriangleInfo {
    pub fn new(
        name: &str,
        material_name: &str,
        point_one: Vec3,
        point_two: Vec3,
        point_three: Vec3,
    ) -> Self {
        return Self {
            name: name.to_string(),
            material_name: material_name.to_string(),
            point_one,
            point_two,
            point_three,
        };
    }

    pub fn boxed(
        name: &str,
        material_name: &str,
        point_one: Vec3,
        point_two: Vec3,
        point_three: Vec3,
    ) -> Box<Self> {
        return Box::new(Self::new(
            name,
            material_name,
            point_one,
            point_two,
            point_three,
        ));
    }
}

impl HittableInfo for TriangleInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_material_name(&self) -> &str {
        return &self.material_name;
    }

    fn build(&self, material: MaterialRc) -> Box<dyn Hittable> {
        return Triangle::boxed(self.point_one, self.point_two, self.point_three, material);
    }
}
