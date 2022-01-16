use crate::hittables::hittable::Hittable;
use crate::materials::material::MaterialRc;

pub trait HittableInfoClone {
    fn clone_box(&self) -> Box<dyn HittableInfo>;
}

pub trait HittableInfo: HittableInfoClone {
    fn get_name(&self) -> &str;
    fn get_material_name(&self) -> &str;
    fn build(&self, material: MaterialRc) -> Box<dyn Hittable>;
}


impl<T> HittableInfoClone for T
    where
        T: HittableInfo + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn HittableInfo> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn HittableInfo> {
    fn clone(&self) -> Self {
        return self.clone_box();
    }
}
