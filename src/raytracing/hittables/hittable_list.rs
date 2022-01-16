use crate::hittables::bounding::aabb::Aabb;
use crate::hittables::hittable::Hittable;
use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;

pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> Self {
        return Self { hittables };
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.hittables {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;

                if closest_so_far > temp_rec.get_distance() {
                    closest_so_far = temp_rec.get_distance();

                    *record = temp_rec.clone();
                }
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self) -> Option<Aabb> {
        todo!();
    }
}
