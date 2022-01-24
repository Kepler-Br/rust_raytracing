extern crate sdl2;

use std::time::Duration;

use nalgebra_glm::{IVec2, UVec2};
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureAccess, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::{Sdl, VideoSubsystem};

use raytracing::image::u8_image_buffer::U8ImageBuffer;

use crate::mainloop::deltatime_calculator::DeltaTimeCalculator;
use crate::mainloop::mainloop_trait::MainLoop;
use crate::mainloop::states::mainloop_state::MainLoopState;

pub struct DefaultMainLoop<T>
where
    T: MainLoopState + Sized,
{
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: WindowCanvas,
    running: bool,
    ticks_passed: u32,
    texture_creator: TextureCreator<WindowContext>,
    texture: Texture,
    resolution: UVec2,
    locked_fps: bool,
    max_fps: u32,
    top_state: T,
}

impl<T> DefaultMainLoop<T>
where
    T: MainLoopState + Sized,
{
    pub fn new(
        resolution: UVec2,
        locked_fps: bool,
        max_fps: u32,
        title: &str,
        top_state: T,
    ) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

        let window = video_subsystem
            .window(title, resolution.x, resolution.y)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().software().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture(
                PixelFormatEnum::BGR888,
                TextureAccess::Streaming,
                resolution.x,
                resolution.y,
            )
            .unwrap();

        return DefaultMainLoop {
            sdl_context,
            video_subsystem,
            canvas,
            running: false,
            ticks_passed: 0,
            texture_creator,
            texture,
            resolution,
            locked_fps,
            max_fps,
            top_state,
        };
    }
}

impl<T> MainLoop for DefaultMainLoop<T>
where
    T: MainLoopState + Sized,
{
    fn draw(&mut self) {
        self.texture
            .with_lock(None, |arr, size| {
                let mut buffer = U8ImageBuffer::new(
                    arr,
                    IVec2::new(self.resolution.x as i32, self.resolution.y as i32),
                    4,
                )
                .unwrap();

                self.top_state.render(&mut buffer);
            })
            .unwrap();

        self.canvas.copy(&self.texture, None, None).unwrap();
    }

    fn update(&mut self, delta_time: f32) {
        self.top_state.update(delta_time);
    }

    fn run(&mut self) {
        self.running = true;

        self.top_state.start();

        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();

        let mut deltatime_calculator = DeltaTimeCalculator::new(self.sdl_context.timer().unwrap());
        let mut delta_sum: f32 = 0.0;

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        const FIXED_TIME_TICK: f32 = 1.0 / 30.0;
        let mut fixed_time_lag = 0.0;

        'running: while self.running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.running = false;
                        break 'running;
                    }
                    _ => {}
                }
            }

            while fixed_time_lag > FIXED_TIME_TICK {
                self.top_state.fixed_update(FIXED_TIME_TICK);
                fixed_time_lag -= FIXED_TIME_TICK;
            }
            self.update(deltatime_calculator.delta_time());
            self.canvas.clear();
            self.draw();
            self.canvas.present();

            self.ticks_passed += 1;

            deltatime_calculator.tick();

            let delta_time = deltatime_calculator.delta_time();

            fixed_time_lag += delta_time;

            delta_sum += delta_time;

            if delta_sum > 1.0 {
                println!("FPS: {}", 1.0 / delta_time);

                delta_sum = 0.0;
            }

            if self.locked_fps && 1000 / self.max_fps > deltatime_calculator.millis() {
                let millis = 1000 / self.max_fps - deltatime_calculator.millis();

                ::std::thread::sleep(Duration::from_millis(millis as u64));
            }
        }
        self.top_state.stop();
    }
}
