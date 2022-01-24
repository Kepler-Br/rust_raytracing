#![allow(dead_code, unused_variables)]
#![allow(clippy::needless_return)]

use nalgebra_glm::IVec2;

use raytracing::examples::cornell_box::cornell_box;

use crate::mainloop::default_mainloop::DefaultMainLoop;
use crate::mainloop::default_mainloop_builder::DefaultMainLoopBuilder;
use crate::mainloop::mainloop_trait::MainLoop;
use crate::mainloop::states::raytracing_state::RaytracingState;

mod mainloop;

pub fn main() {
    let resolution = IVec2::new(800, 600);
    let tracer_resolution = resolution;

    let scene_info = cornell_box(resolution);

    let mut mainloop = DefaultMainLoopBuilder::new()
        .state(RaytracingState::new(
            scene_info,
            tracer_resolution,
            8,
            Option::Some("result.ppm"),
        ))
        .title("Raytracing")
        .resolution(resolution.x as u32, resolution.y as u32)
        .max_fps(30)
        .locked_fps(true)
        .build();

    mainloop.run();
}
