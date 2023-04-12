use gl::types::GLuint;

pub struct SSBO {
    id: GLuint,
}

#[allow(clippy::new_without_default)]
impl SSBO {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        SSBO { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    pub fn bind_buffer_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                (data.len() * std::mem::size_of::<f32>())
                    as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    pub fn bind_buffer_base(&self, index: u32) {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, self.id);
        }
    }

    pub fn bind_buffer_sub_data(&self, offset: isize, data: &[f32]) {
        unsafe {
            gl::BufferSubData(
                gl::SHADER_STORAGE_BUFFER,
                offset,
                (data.len() * std::mem::size_of::<f32>())
                    as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
            );
        }
    }
}

impl Drop for SSBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
