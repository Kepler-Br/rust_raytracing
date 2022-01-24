use nalgebra_glm::IVec2;

use raytracing::image::buffer_converter::BufferConverter;
use raytracing::image::default_image_buffer::DefaultImageBuffer;
use raytracing::image::image_buffer::ImageBuffer;
use raytracing::image::ppm_converter::PpmConverter;
use raytracing::scene::scene_info::SceneInfo;
use raytracing::thread_pool::pool::ThreadPool;

use crate::mainloop::states::mainloop_state::MainLoopState;

pub struct RaytracingState {
    scene_info: SceneInfo,
    resolution: IVec2,
    total_samples: u64,
    pool: ThreadPool,
    buffer: DefaultImageBuffer,
    threads: usize,
    result_path: Option<String>,
}

impl RaytracingState {
    pub fn new(
        scene: SceneInfo,
        resolution: IVec2,
        threads: usize,
        result_path: Option<&str>,
    ) -> Self {
        let path;

        if let Some(some_path) = result_path {
            path = Option::Some(some_path.to_string());
        } else {
            path = Option::None;
        }

        return RaytracingState {
            scene_info: scene,
            resolution,
            total_samples: 0,
            pool: ThreadPool::new(threads),
            buffer: DefaultImageBuffer::new(resolution).unwrap(),
            threads,
            result_path: path,
        };
    }
}

impl MainLoopState for RaytracingState {
    fn start(&mut self) {
        self.pool
            .execute_scene(self.scene_info.clone(), self.threads);
    }

    fn stop(&mut self) {
        if let Some(path) = &self.result_path {
            match PpmConverter::new(path) {
                Ok(ppm) => {
                    if let Err(err) =
                        ppm.convert_with(&self.buffer, |color| color / self.total_samples as f32)
                    {
                        println!("Error writing result: {}", err);
                    }
                }
                Err(err) => println!("Error writing result: {}", err),
            }
        }
    }

    fn update(&mut self, delta_time: f32) {
        // self.tracer.trace();
        if let Some(trace_result) = self.pool.try_receive() {
            trace_result.get_image().add_to(&mut self.buffer).unwrap();

            self.total_samples += trace_result.get_samples();
            println!("Total samples: {}", self.total_samples);
        }
    }

    fn fixed_update(&mut self, delta_time: f32) {}

    fn render<T>(&mut self, buffer: &mut T)
    where
        T: ImageBuffer + Sized,
    {
        let total_samples_div = 1.0 / self.total_samples as f32;

        self.buffer
            .copy_to_with(buffer, |color, _, _| {
                color.apply_into(|s| *s = f32::sqrt(*s * total_samples_div))
            })
            .unwrap();
        // self.tracer
        //     .get_buffer()
        //     .scale_copy_to_with(buffer, |color, _| {
        //         color.div(self.tracer.get_total_samples() as f32)
        //     })
        //     .unwrap();
    }
}
