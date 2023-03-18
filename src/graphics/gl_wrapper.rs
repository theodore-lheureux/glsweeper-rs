use std::ffi::c_void;

use gl::types::{GLboolean, GLenum, GLfloat, GLsizei};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VAO {
    id: gl::types::GLuint,
}

impl VAO {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        // info!("VAO created. (id: {})", id);
        VAO { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
        // info!("VAO dropped. (id: {})", self.id);
    }
}

pub struct VBO {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl VBO {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        VBO { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn bind_buffer_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * std::mem::size_of::<GLfloat>())
                    as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            );
        }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
        // info!("VBO dropped. (id: {})", self.id);
    }
}

pub struct EBO {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl EBO {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        EBO { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn bind_buffer_data(&self, data: &[u32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * std::mem::size_of::<u32>())
                    as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                self.usage,
            );
        }
    }
}

impl Drop for EBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
        // info!("EBO dropped. (id: {})", self.id);
    }
}

pub struct VertexAttribute {
    pub index: gl::types::GLuint,
}

impl VertexAttribute {
    pub unsafe fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> Self {
        gl::VertexAttribPointer(
            index, size, r#type, normalized, stride, pointer,
        );

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

impl Drop for VertexAttribute {
    fn drop(&mut self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
        // info!("Dropping VertexAttribute: {}", self.index);
    }
}
