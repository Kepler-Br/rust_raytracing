use std::fs::File;
use std::io::{BufWriter, Write};

use nalgebra_glm::Vec3;

use crate::image::buffer_converter::BufferConverter;
use crate::image::image_buffer::ImageBuffer;
use crate::misc::color::vec3_to_ivec3;

pub struct PpmConverter {
    path: String,
}

impl PpmConverter {
    pub fn new(path: &str) -> Result<Self, String> {
        if path.is_empty() {
            return Result::Err(format!("{path} is not a correct path"));
        }

        return Result::Ok(Self {
            path: path.to_string(),
        });
    }
}

impl PpmConverter {
    pub fn set_output_file(&mut self, path: &str) {
        self.path = path.to_string();
    }
}

impl BufferConverter for PpmConverter {
    fn convert(&self, buffer: &impl ImageBuffer) -> Result<(), String> {
        let file = File::create(self.path.clone())
            .map_err(|x| format!("Error creating file: {}. {}", self.path, x))?;
        let mut buffered_writer = BufWriter::new(file);

        writeln!(
            buffered_writer,
            "P3\n{} {}\n255",
            buffer.get_resolution().x,
            buffer.get_resolution().y
        )
        .map_err(|x| format!("Error writing to file: {}. {}", self.path, x))?;

        for i in buffer.get_buffer() {
            let int_color = vec3_to_ivec3(i);

            writeln!(
                buffered_writer,
                "{} {} {}",
                int_color.x, int_color.y, int_color.z
            )
            .map_err(|x| format!("Error writing to file: {}. {}", self.path, x))?;
        }

        return Result::Ok(());
    }

    fn convert_with<F>(&self, buffer: &impl ImageBuffer, preprocess: F) -> Result<(), String>
    where
        F: Fn(&Vec3) -> Vec3,
    {
        let file = File::create(self.path.clone())
            .map_err(|x| format!("Error creating file: {}. {}", self.path, x))?;
        let mut buffered_writer = BufWriter::new(file);

        writeln!(
            buffered_writer,
            "P3\n{} {}\n255",
            buffer.get_resolution().x,
            buffer.get_resolution().y
        )
        .map_err(|x| format!("Error writing to file: {}. {}", self.path, x))?;

        for i in buffer.get_buffer() {
            let processed_color = preprocess(i);

            let int_color = vec3_to_ivec3(&processed_color);

            writeln!(
                buffered_writer,
                "{} {} {}",
                int_color.x, int_color.y, int_color.z
            )
            .map_err(|x| format!("Error writing to file: {}. {}", self.path, x))?;
        }

        return Result::Ok(());
    }
}
