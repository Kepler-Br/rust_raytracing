use raytracing::image::image_buffer::ImageBuffer;

pub trait MainLoopState {
    fn start(&mut self);
    fn stop(&mut self);
    fn update(&mut self, delta_time: f32);
    fn fixed_update(&mut self, delta_time: f32);
    fn render<T>(&mut self, buffer: &mut T)
    where
        T: ImageBuffer + Sized;
}
