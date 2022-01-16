use crate::image::default_image_buffer::DefaultImageBuffer;

pub struct TraceResult {
    image: DefaultImageBuffer,
    samples: u64,
}

unsafe impl Send for TraceResult {}

impl TraceResult {
    pub fn new(image: DefaultImageBuffer, samples: u64) -> Self {
        return Self {
            image,
            samples,
        };
    }
}

impl TraceResult {
    pub fn get_image(&self) -> &DefaultImageBuffer {
        return &self.image;
    }

    pub fn get_samples(&self) -> u64 {
        return self.samples;
    }
}
