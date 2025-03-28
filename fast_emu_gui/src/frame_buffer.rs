use eframe::egui::ColorImage;

use crate::FastEmuGUIError;

pub(crate) struct FrameBuffer {
    height: usize,
    width: usize,
    required_buffer_length: usize,
    image: ColorImage,
}

impl FrameBuffer {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            required_buffer_length: height * width * 4,
            image: Default::default(),
        }
    }
    pub fn update_frame_buffer(&mut self, buffer: &[u8]) -> Result<(), FastEmuGUIError> {
        if buffer.len() != self.required_buffer_length {
            return Err(FastEmuGUIError::MismatchedBufferSize {
                expected: self.required_buffer_length,
                received: buffer.len(),
            });
        }

        for value in buffer {
            if *value != 255 {
                // println!("value: {}", value)
            }
        }

        self.image = ColorImage::from_rgba_unmultiplied([self.width, self.height], buffer);

        Ok(())
    }

    pub fn get_image(&self) -> &ColorImage {
        &self.image
    }
}
