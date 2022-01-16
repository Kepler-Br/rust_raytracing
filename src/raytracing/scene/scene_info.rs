use std::collections::HashMap;
use std::sync::Arc;

use nalgebra_glm::Vec3;

use crate::hittables::hittable_list::HittableList;
use crate::misc::rand_gen::RandGenRc;
use crate::misc::scene::Scene;
use crate::scene::camera_info::CameraInfo;
use crate::scene::emission_info::EmissionInfo;
use crate::scene::hittable_info::HittableInfo;
use crate::scene::lambertian_info::LambertianInfo;
use crate::scene::material_info::MaterialInfo;
use crate::scene::reflective_info::ReflectiveInfo;
use crate::scene::refractive_info::RefractiveInfo;
use crate::scene::sphere_info::SphereInfo;
use crate::scene::triangle_info::TriangleInfo;
use crate::scene::xy_rect_info::XyRectInfo;
use crate::scene::xz_rect_info::XzRectInfo;
use crate::scene::yz_rect_info::YzRectInfo;

#[derive(Clone)]
pub struct SceneInfo {
    camera: Option<CameraInfo>,
    materials: HashMap<String, Box<dyn MaterialInfo>>,
    hittables: Vec<Box<dyn HittableInfo>>,
    rand_producer: Arc<Box<dyn Fn() -> RandGenRc>>,
}

unsafe impl Send for SceneInfo {}

impl SceneInfo {
    pub fn new<T>(rand_producer: T) -> Self
        where
            T: Fn() -> RandGenRc + Send + 'static,
    {
        return Self {
            camera: Option::None,
            materials: HashMap::new(),
            hittables: Vec::new(),
            rand_producer: Arc::new(Box::new(rand_producer)),
        };
    }
}

impl SceneInfo {
    pub fn camera(
        mut self,
        origin: Vec3,
        center: Vec3,
        up: Vec3,
        v_fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        self.camera = Option::Some(CameraInfo::new(origin, center, up, v_fov, aspect_ratio));

        return self;
    }

    pub fn sphere(mut self, name: &str, material_name: &str, center: Vec3, radius: f32) -> Self {
        self.hittables
            .push(SphereInfo::boxed(name, material_name, center, radius));

        return self;
    }

    pub fn xy_square(
        mut self,
        name: &str,
        material_name: &str,
        position: Vec3,
        edge_length: f32,
    ) -> Self {
        self.hittables.push(XyRectInfo::square_boxed(
            name,
            material_name,
            position,
            edge_length,
        ));

        return self;
    }

    pub fn xz_square(
        mut self,
        name: &str,
        material_name: &str,
        position: Vec3,
        edge_length: f32,
    ) -> Self {
        self.hittables.push(XzRectInfo::square_boxed(
            name,
            material_name,
            position,
            edge_length,
        ));

        return self;
    }

    pub fn yz_square(
        mut self,
        name: &str,
        material_name: &str,
        position: Vec3,
        edge_length: f32,
    ) -> Self {
        self.hittables.push(YzRectInfo::square_boxed(
            name,
            material_name,
            position,
            edge_length,
        ));

        return self;
    }

    pub fn triangle(
        mut self,
        name: &str,
        material_name: &str,
        point_one: Vec3,
        point_two: Vec3,
        point_three: Vec3,
    ) -> Self {
        self.hittables.push(TriangleInfo::boxed(
            name,
            material_name,
            point_one,
            point_two,
            point_three,
        ));

        return self;
    }

    pub fn emission(mut self, name: &str, color: Vec3, power: f32) -> Self {
        self.materials
            .insert(name.to_string(), EmissionInfo::boxed(name, color, power));

        return self;
    }

    pub fn lambertian(mut self, name: &str, color: Vec3) -> Self {
        self.materials
            .insert(name.to_string(), LambertianInfo::boxed(name, color));

        return self;
    }

    pub fn reflective(mut self, name: &str, color: Vec3, reflectiveness: f32) -> Self {
        self.materials.insert(
            name.to_string(),
            ReflectiveInfo::boxed(name, color, reflectiveness),
        );

        return self;
    }

    pub fn refractive(mut self, name: &str, color: Vec3, index_of_refraction: f32) -> Self {
        self.materials.insert(
            name.to_string(),
            RefractiveInfo::boxed(name, color, index_of_refraction),
        );

        return self;
    }

    pub fn build(self) -> Scene {
        let rand = (self.rand_producer)();
        let camera = self.camera.expect("Camera is required").build(rand.clone());
        let mut materials = Vec::with_capacity(self.materials.len());
        let mut hittables = Vec::with_capacity(self.hittables.len());
        self.materials
            .values()
            .for_each(|x| materials.push(x.build(rand.clone())));
        for h in &self.hittables {
            let material_name = h.get_material_name();
            let material = self
                .materials
                .get(material_name)
                .unwrap_or_else(|| panic!("Cannot find material with name \"{}\"", material_name));

            hittables.push(h.build(material.build(rand.clone())));
        }

        return Scene::new(camera, materials, HittableList::new(hittables));
    }
}

// impl SceneBuilder {
//     pub fn hittable(mut self, hittable: Box<dyn Hittable>) -> Self {
//         self.hittables.push(hittable);
//
//         return self;
//     }
//
//     pub fn material(mut self, material: MaterialRc) -> Self {
//         self.materials.push(material);
//
//         return self;
//     }
//
//     pub fn build(self) -> Scene {
//         return Scene::new(
//             self.camera.expect("Camera is needed"),
//             self.materials,
//             HittableList::new(self.hittables),
//         );
//     }
// }
