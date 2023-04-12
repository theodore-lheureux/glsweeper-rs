use log::info;

pub struct Texture {
    id: gl::types::GLuint,
    unit: Option<u32>,
}

impl Texture {
    pub fn new(image_file: Vec<u8>, unit: u32) -> Self {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, id);
        }

        let mut texture = Texture {
            id,
            unit: Some(unit),
        };

        unsafe {
            gl::TexStorage3D(gl::TEXTURE_2D_ARRAY, 3, gl::RGB8, 32, 32, 14);

            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );

            for i in 0..14 {
                gl::TexSubImage3D(
                    gl::TEXTURE_2D_ARRAY,
                    0,
                    0,
                    0,
                    i,
                    32,
                    32,
                    1,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    image_file.as_ptr().add((i * 32 * 32 * 3) as usize)
                        as *const _,
                );
            }
        }

        texture.unbind();

        texture
    }

    pub fn bind(&mut self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.id);
            self.unit = Some(unit);
        }
    }

    pub fn unbind(&mut self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit.unwrap());
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0);
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
