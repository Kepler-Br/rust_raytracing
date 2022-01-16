use nalgebra_glm::IVec2;

use raytracing::image::default_image_buffer::DefaultImageBuffer;
use raytracing::image::image_buffer::ImageBuffer;
use raytracing::scene::scene_info::SceneInfo;
use raytracing::thread_pool::pool::ThreadPool;

use crate::mainloop::states::mainloop_state::MainLoopState;

pub struct RaytracingState {
    scene_info: SceneInfo,
    resolution: IVec2,
    total_samples: u64,
    pool: ThreadPool,
    buffer: DefaultImageBuffer,
}

impl RaytracingState {
    pub fn new(scene: SceneInfo, resolution: IVec2) -> Self {
        return RaytracingState {
            scene_info: scene,
            resolution,
            total_samples: 0,
            pool: ThreadPool::new(16),
            buffer: DefaultImageBuffer::new(resolution).unwrap(),
        };
    }
}

impl MainLoopState for RaytracingState {
    fn start(&mut self) {
        let clone_scene = self.scene_info.clone();
        // self.pool.execute(move || )
        // let resolution = IVec2::new(800, 600);
        // let scene = ;
        // let mut tracer = Tracer::new(
        //     IVec2::new(800, 600),
        //     cornell_box(IVec2::new(800, 600)).build(),
        // );
        // let aaaa =  cornell_box(IVec2::new(800, 600));

        self.pool.execute_scene(clone_scene.clone(), 16);
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
