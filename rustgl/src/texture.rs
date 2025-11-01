use gl::types::*;
use std::path::Path;
use image::{DynamicImage, GenericImageView};

pub struct Texture {
    pub id: GLuint,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(path: &str) -> Result<Self, String> {
        // 1. Load image from disk
        let img = image::open(Path::new(path))
            .map_err(|e| format!("Failed to load texture {}: {}", path, e))?;

        // 2. Convert to RGBA8 format (required by OpenGL)
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();
        let data = img.into_raw();

        // 3. Generate OpenGL texture
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // 4. Upload pixel data to GPU
            gl::TexImage2D(
                gl::TEXTURE_2D,           // Target
                0,                        // Mipmap level (0 = base)
                gl::RGBA as GLint,        // Internal format
                width as GLint,
                height as GLint,
                0,                        // Border (must be 0)
                gl::RGBA,                 // Format of data
                gl::UNSIGNED_BYTE,        // Type of data
                data.as_ptr() as *const _, // Pointer to data
            );

            // 5. Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            // 6. Generate mipmaps
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture { id, width, height })
    }

    pub fn bind(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
