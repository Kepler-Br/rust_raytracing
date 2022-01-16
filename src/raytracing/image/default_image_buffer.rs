use nalgebra_glm::{IVec2, Vec3};

use crate::image::image_buffer::ImageBuffer;

pub struct DefaultImageBuffer {
    buffer: Vec<Vec3>,
    resolution: IVec2,
}

unsafe impl Send for DefaultImageBuffer {}

impl DefaultImageBuffer {
    pub fn new(resolution: IVec2) -> Result<Self, String> {
        if resolution.x <= 0 || resolution.y <= 0 {
            return Result::Err(format!(
                "Negative resolution: x: {}, y: {}",
                resolution.x, resolution.y
            ));
        }

        let buffer_size = (resolution.x * resolution.y) as usize;

        let mut buffer: Vec<Vec3> = Vec::with_capacity(buffer_size);

        buffer.resize(buffer_size, Vec3::new(0.0, 0.0, 0.0));

        return Result::Ok(DefaultImageBuffer { buffer, resolution });
    }
}

impl DefaultImageBuffer {
    fn check_if_out_of_bounds(&self, coord: &IVec2) -> Result<(), String> {
        if coord.x < 0 || coord.y < 0 {
            return Result::Err(format!(
                "Out of bounds: x: {} < 0, y: {} < 0",
                coord.x, coord.y
            ));
        }

        if coord.x >= self.resolution.x || coord.y >= self.resolution.y {
            return Result::Err(format!(
                "Out of bounds: x: {} >= {}, y: {} >= {}",
                coord.x, self.resolution.x, coord.y, self.resolution.y
            ));
        }

        return Result::Ok(());
    }

    fn calc_index(&self, coord: &IVec2) -> usize {
        return coord.y as usize * self.resolution.x as usize + coord.x as usize;
    }
}

impl ImageBuffer for DefaultImageBuffer {
    fn add(&mut self, coord: IVec2, color: Vec3) -> Result<(), String> {
        self.check_if_out_of_bounds(&coord)?;

        let index = self.calc_index(&coord);

        self.buffer[index] += color;

        return Result::Ok(());
    }

    fn put(&mut self, coord: IVec2, color: Vec3) -> Result<(), String> {
        self.check_if_out_of_bounds(&coord)?;

        let index = self.calc_index(&coord);

        self.buffer[index] = color;

        return Result::Ok(());
    }

    fn get(&self, coord: IVec2) -> Result<Vec3, String> {
        self.check_if_out_of_bounds(&coord)?;

        let index = self.calc_index(&coord);

        return Result::Ok(self.buffer[index]);
    }

    fn get_resolution(&self) -> IVec2 {
        return self.resolution;
    }

    fn get_buffer(&self) -> &[Vec3] {
        return &self.buffer[..];
    }

    fn get_buffer_mut(&mut self) -> &mut [Vec3] {
        return &mut self.buffer[..];
    }
}
