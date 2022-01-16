use nalgebra_glm::{IVec2, Vec3};

use crate::image::image_buffer::ImageBuffer;

pub struct U8ImageBuffer<'a> {
    resolution: IVec2,
    buffer: &'a mut [u8],
    channels: i32,
}

impl<'a> U8ImageBuffer<'a> {
    pub fn new(buffer: &'a mut [u8], resolution: IVec2, channels: u32) -> Result<Self, String> {
        if resolution.x <= 0 || resolution.y <= 0 {
            return Result::Err(format!(
                "Negative resolution: x: {}, y: {}",
                resolution.x, resolution.y
            ));
        }

        let total_possible_pixels =
            resolution.x as usize * resolution.y as usize * channels as usize;

        if total_possible_pixels > buffer.len() {
            return Result::Err(format!(
                "Total possible pixel count is greater than buffer len: {} > {}",
                total_possible_pixels,
                buffer.len()
            ));
        }

        return Result::Ok(U8ImageBuffer {
            resolution,
            buffer,
            channels: channels as i32,
        });
    }
}

impl<'a> U8ImageBuffer<'a> {
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
        return (coord.y as usize * self.resolution.x as usize + coord.x as usize)
            * self.channels as usize;
    }
}

impl<'a> ImageBuffer for U8ImageBuffer<'a> {
    fn add(&mut self, coord: IVec2, color: Vec3) -> Result<(), String> {
        self.check_if_out_of_bounds(&coord)?;

        let index = self.calc_index(&coord);

        let value = &mut self.buffer[index];
        *value = value.saturating_add((color.x.clamp(0.0, 1.0) * 255.0) as u8);

        let value = &mut self.buffer[index + 1];
        *value = value.saturating_add((color.y.clamp(0.0, 1.0) * 255.0) as u8);

        let value = &mut self.buffer[index + 2];
        *value = value.saturating_add((color.z.clamp(0.0, 1.0) * 255.0) as u8);

        return Result::Ok(());
    }

    fn put(&mut self, coord: IVec2, color: Vec3) -> Result<(), String> {
        self.check_if_out_of_bounds(&coord)?;

        let index = self.calc_index(&coord);

        self.buffer[index] = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
        self.buffer[index + 1] = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
        self.buffer[index + 2] = (color.z.clamp(0.0, 1.0) * 255.0) as u8;

        return Result::Ok(());
    }

    fn get(&self, coord: IVec2) -> Result<Vec3, String> {
        self.check_if_out_of_bounds(&coord)?;

        let index = self.calc_index(&coord);

        const DIV: f32 = 1.0 / 255.0;

        return Result::Ok(Vec3::new(
            self.buffer[index] as f32 * DIV,
            self.buffer[index + 1] as f32 * DIV,
            self.buffer[index + 2] as f32 * DIV,
        ));
    }

    fn get_resolution(&self) -> IVec2 {
        return self.resolution;
    }

    fn get_buffer(&self) -> &[Vec3] {
        panic!("get_buffer is not implemented for U8ImageBuffer");
    }

    fn get_buffer_mut(&mut self) -> &mut [Vec3] {
        panic!("get_buffer_mut is not implemented for U8ImageBuffer");
    }
}
