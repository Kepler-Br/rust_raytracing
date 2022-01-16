use crate::hittables::hittable_list::HittableList;
use crate::materials::material::MaterialRc;
use crate::misc::camera::Camera;

pub struct Scene {
    camera: Camera,
    materials: Vec<MaterialRc>,
    hittable_list: HittableList,
}

impl Scene {
    pub fn new(camera: Camera, materials: Vec<MaterialRc>, hittable_list: HittableList) -> Self {
        return Scene {
            camera,
            materials,
            hittable_list,
        };
    }
}

impl Scene {
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    pub fn get_hittable_list_mut(&mut self) -> &mut HittableList {
        return &mut self.hittable_list;
    }

    pub fn get_materials_mut(&mut self) -> &mut Vec<MaterialRc> {
        return &mut self.materials;
    }

    pub fn get_camera(&self) -> &Camera {
        return &self.camera;
    }

    pub fn get_hittable_list(&self) -> &HittableList {
        return &self.hittable_list;
    }

    pub fn get_materials(&self) -> &Vec<MaterialRc> {
        return &self.materials;
    }
}
