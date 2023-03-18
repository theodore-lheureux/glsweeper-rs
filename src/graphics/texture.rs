use std::path::Path;

use image::EncodableLayout;
use log::info;

pub struct Texture {
    id: gl::types::GLuint,
    unit: Option<u32>,
}

impl Texture {
    pub fn new(image_file: Vec<u8>, unit: u32) -> Self {
        let img = image::load_from_memory(&image_file).expect("Failed to load texture image.");
        let rgba = img.into_rgba8();
        let (width, height) = rgba.dimensions();

        info!("Texture loaded. (width: {}, height: {})", width, height);

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, id);
        }

        let mut texture = Self {
            id,
            unit: Some(unit),
        };

        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                rgba.as_bytes().as_ptr() as *const gl::types::GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        texture.unbind();

        info!("Texture created. (id: {})", id);

        texture
    }

    pub fn bind(&mut self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            self.unit = Some(unit);
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit.unwrap());
            gl::BindTexture(gl::TEXTURE_2D, 0);
            self.unit = None;
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
        info!("Texture dropped. (id: {})", self.id);
    }
}
