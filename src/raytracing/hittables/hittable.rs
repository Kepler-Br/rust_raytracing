use crate::hittables::bounding::aabb::Aabb;
use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;

pub trait Hittable {
    ///
    ///
    /// # Arguments
    ///
    /// * `ray`:
    /// * `t_min`:
    /// * `t_max`:
    /// * `record`:
    ///
    /// returns: bool If true - hit, false - miss
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Option<Aabb>;
}
