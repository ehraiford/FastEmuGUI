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
    pub fn update_frame_buffer(&mut self, buffer: &[u8], mutex: MutexWrapper) -> Result<(), FastEmuGUIError> {
        if buffer.len() != self.required_buffer_length {
            return Err(FastEmuGUIError::MismatchedBufferSize {
                expected: self.required_buffer_length,
                received: buffer.len(),
            });
        }

        unsafe {
            lock(&mutex as *const _ as *mut MutexWrapper);
        };
        self.image = ColorImage::from_rgba_unmultiplied([self.width, self.height], buffer);
        unsafe {
            unlock(&mutex as *const _ as *mut MutexWrapper);
        };

        Ok(())
    }

    pub fn get_image(&self) -> &ColorImage {
        &self.image
    }
}

#[repr(C)]
/// Frustratingly, Rust has a very different structure for its mutexes compared to most other languages.
/// Because of this, we can't just pass them around from C++ to Rust.
/// Instead, we are defining a wrapper around a mutex with functions for handling it.
/// Then, the actual definition of the struct is done in the connected language and Rust just uses that.
pub struct MutexWrapper {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub unsafe fn create() -> *mut MutexWrapper;
    pub unsafe fn lock(m: *mut MutexWrapper);
    pub unsafe fn unlock(m: *mut MutexWrapper);
    pub unsafe fn destroy(m: *mut MutexWrapper);
}
