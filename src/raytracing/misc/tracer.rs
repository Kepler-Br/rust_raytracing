use nalgebra_glm::{IVec2, Vec3};

use crate::hittables::hittable::Hittable;
use crate::image::default_image_buffer::DefaultImageBuffer;
use crate::image::image_buffer::ImageBuffer;
use crate::misc::hit_record::HitRecord;
use crate::misc::ray::Ray;
use crate::misc::scene::Scene;
use crate::thread_pool::trace_result::TraceResult;

pub struct Tracer {
    buffer: DefaultImageBuffer,
    total_samples: u64,
    scene: Scene,
}

impl Tracer {
    pub fn new(resolution: IVec2, scene: Scene) -> Self {
        return Tracer {
            buffer: DefaultImageBuffer::new(resolution).unwrap(),
            total_samples: 0,
            scene,
        };
    }
}

impl Tracer {
    pub fn trace(&mut self) {
        let hittable_list = self.scene.get_hittable_list();
        let camera = self.scene.get_camera();
        let buffer = &mut self.buffer;
        let resolution = buffer.get_resolution();

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                let norm_x = x as f32 / resolution.x as f32;
                let norm_y = y as f32 / resolution.y as f32;

                let mut ray = camera.get_ray(norm_x, norm_y);
                let mut absorbed = false;
                let mut result_color = Vec3::new(1.0, 1.0, 1.0);
                let t_min = 0.0001;
                let t_max = f32::INFINITY;
                const DEPTH: u32 = 20;
                let world_color: Vec3 = Vec3::new(1.0, 1.0, 1.0) * 0.0;

                for cur_depth in 0..DEPTH {
                    let mut record = HitRecord::default();

                    if hittable_list.hit(&ray, t_min, t_max, &mut record) {
                        if let Some(material) = record.get_material() {
                            let mut scattered_ray = Ray::default();
                            let mut attenuation = Vec3::default();

                            if material.borrow_mut().scatter(
                                &ray,
                                &record,
                                &mut attenuation,
                                &mut scattered_ray,
                            ) {
                                ray = scattered_ray;

                                result_color.component_mul_assign(&attenuation);
                            } else {
                                result_color.component_mul_assign(&attenuation);
                                absorbed = true;

                                break;
                            }
                        } else {
                            result_color.component_mul_assign(&world_color);
                            absorbed = true;

                            break;
                        }
                    } else {
                        result_color.component_mul_assign(&world_color);
                        absorbed = true;

                        break;
                    }
                }

                if !absorbed {
                    result_color.component_mul_assign(&Vec3::default());
                }

                buffer.add(IVec2::new(x, y), result_color).unwrap();
            }
        }

        self.total_samples += 1;
    }

    pub fn get_buffer(&self) -> &impl ImageBuffer {
        return &self.buffer;
    }

    pub fn get_total_samples(&self) -> u64 {
        return self.total_samples;
    }

    pub fn construct_trace_result(&mut self) -> TraceResult {
        let total_samples = std::mem::take(&mut self.total_samples);
        let resolution = self.buffer.get_resolution();
        let buffer = std::mem::replace(
            &mut self.buffer,
            DefaultImageBuffer::new(resolution).unwrap(),
        );

        return TraceResult::new(buffer, total_samples);
    }
}
