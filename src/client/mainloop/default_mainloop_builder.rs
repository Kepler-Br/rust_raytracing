use nalgebra_glm::UVec2;

use crate::DefaultMainLoop;
use crate::mainloop::states::mainloop_state::MainLoopState;

pub struct DefaultMainLoopBuilder<T>
    where
        T: MainLoopState + Sized,
{
    locked_fps: bool,
    max_fps: u32,
    resolution: Option<UVec2>,
    title: Option<String>,
    top_state: Option<T>,
}

impl<T> DefaultMainLoopBuilder<T>
    where
        T: MainLoopState + Sized,
{
    pub fn new() -> Self {
        return DefaultMainLoopBuilder {
            locked_fps: true,
            max_fps: 60,
            resolution: Option::None,
            title: Option::None,
            top_state: Option::None,
        };
    }
}

impl<T> DefaultMainLoopBuilder<T>
    where
        T: MainLoopState + Sized,
{
    pub fn locked_fps(mut self, is_locked: bool) -> Self {
        self.locked_fps = is_locked;

        return self;
    }

    pub fn max_fps(mut self, max_fps: u32) -> Self {
        self.max_fps = max_fps;

        return self;
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Option::Some(title.to_string());

        return self;
    }

    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.resolution = Option::Some(UVec2::new(width, height));

        return self;
    }

    pub fn state(mut self, state: T) -> Self {
        self.top_state = Option::Some(state);

        return self;
    }

    pub fn build(self) -> DefaultMainLoop<T> {
        return DefaultMainLoop::new(
            self.resolution.unwrap_or_else(|| UVec2::new(800, 600)),
            self.locked_fps,
            self.max_fps,
            &self.title
                .unwrap_or_else(|| "SDL2 window".to_string()),
            self.top_state.expect("State is required"),
        );
    }
}
