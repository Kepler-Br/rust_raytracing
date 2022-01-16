use nalgebra_glm::Vec3;

use crate::materials::material::MaterialRc;

#[derive(Clone)]
pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    distance: f32,
    material: Option<MaterialRc>,
    is_front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, is_front_face: bool, material: MaterialRc) -> Self {
        return Self {
            point,
            normal,
            distance,
            material: Option::Some(material),
            is_front_face
        };
    }

    pub fn default() -> Self {
        return Self {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            distance: 0.0,
            material: Option::None,
            is_front_face: true,
        };
    }
}

impl HitRecord {
    pub fn get_point(&self) -> &Vec3 {
        return &self.point;
    }

    pub fn get_normal(&self) -> &Vec3 {
        return &self.normal;
    }

    pub fn get_distance(&self) -> f32 {
        return self.distance;
    }

    pub fn get_material(&self) -> Option<MaterialRc> {
        return self.material.clone();
    }

    pub fn get_is_front_face(&self) -> bool {
        return self.is_front_face;
    }

    pub fn set_point(&mut self, point: Vec3) {
        self.point = point;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }

    pub fn set_material(&mut self, material: MaterialRc) {
        self.material = Option::Some(material);
    }

    pub fn set_is_front_face(&mut self, is_front_face: bool) {
        self.is_front_face = is_front_face;
    }
}
