use gl::types::*;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

pub enum TextureType {
    Texture2D,
    Cubemap,
}

pub struct Texture {
    pub id: GLuint,
    pub width: u32,
    pub height: u32,
    pub texture_type: TextureType,
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
                gl::TEXTURE_2D,    // Target
                0,                 // Mipmap level (0 = base)
                gl::RGBA as GLint, // Internal format
                width as GLint,
                height as GLint,
                0,                         // Border (must be 0)
                gl::RGBA,                  // Format of data
                gl::UNSIGNED_BYTE,         // Type of data
                data.as_ptr() as *const _, // Pointer to data
            );

            // 5. Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            // 6. Generate mipmaps
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture { id, width, height, texture_type: TextureType::Texture2D })
    }

    /// Load a cubemap texture from 6 separate image files
    /// Order: right, left, top, bottom, front, back (+X, -X, +Y, -Y, +Z, -Z)
    pub fn new_cubemap(faces: [&str; 6]) -> Result<Self, String> {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

            // Load each face
            for (i, face_path) in faces.iter().enumerate() {
                let img = image::open(face_path)
                    .map_err(|e| format!("Failed to load cubemap face {}: {}", face_path, e))?;

                // Don't flip cubemap textures - they're already in the correct orientation
                let data = img.to_rgb8();
                let (width, height) = img.dimensions();

                // GL_TEXTURE_CUBE_MAP_POSITIVE_X + i gives us each face
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    gl::RGB as i32,
                    width as i32,
                    height as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const _,
                );
            }

            // Cubemap texture parameters
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }

        Ok(Texture {
            id: texture_id,
            width: 0,  // Not really relevant for cubemaps
            height: 0,
            texture_type: TextureType::Cubemap,
        })
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            match self.texture_type {
                TextureType::Texture2D => gl::BindTexture(gl::TEXTURE_2D, self.id),
                TextureType::Cubemap => gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id),
            }
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
