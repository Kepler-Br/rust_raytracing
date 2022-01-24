use crate::materials::material::MaterialRc;
use crate::misc::rand_gen::RandGenRc;

pub trait MaterialInfo: MaterialInfoClone {
    fn get_name(&self) -> &str;
    fn build(&self, rand: RandGenRc) -> MaterialRc;
}

pub trait MaterialInfoClone {
    fn clone_box(&self) -> Box<dyn MaterialInfo>;
}

impl<T> MaterialInfoClone for T
where
    T: MaterialInfo + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn MaterialInfo> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MaterialInfo> {
    fn clone(&self) -> Self {
        return self.clone_box();
    }
}
