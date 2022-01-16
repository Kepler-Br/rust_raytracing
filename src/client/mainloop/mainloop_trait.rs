pub trait MainLoop {
    fn draw(&mut self);
    fn update(&mut self, delta_time: f32);
    fn run(&mut self);
}
