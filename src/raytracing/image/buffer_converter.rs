use nalgebra_glm::Vec3;

use crate::image::image_buffer::ImageBuffer;

pub trait BufferConverter {
    fn convert(&self, buffer: &impl ImageBuffer) -> Result<(), String>;
    fn convert_with<F>(&self, buffer: &impl ImageBuffer, preprocess: F) -> Result<(), String>
    where
        F: Fn(&Vec3) -> Vec3;
}
