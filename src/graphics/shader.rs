use std::ffi::CString;

use log::info;

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_code: String, fragment_code: String) -> Self {
        let vertex_shader =
            Self::compile_shader(vertex_code.as_str(), gl::VERTEX_SHADER);
        let fragment_shader =
            Self::compile_shader(fragment_code.as_str(), gl::FRAGMENT_SHADER);

        let id = Self::link_program(vertex_shader, fragment_shader);

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Self { id }
    }

    fn compile_shader(source: &str, shader_type: u32) -> u32 {
        let id = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            panic!("{}", error.to_string_lossy().into_owned());
        }

        info!("Shader compiled successfully. (id: {})", id);

        id
    }

    fn link_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);
        }

        let mut success = 0;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            panic!("{}", error.to_string_lossy().into_owned());
        }

        info!("Program linked successfully. (id: {})", id);

        id
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                value as i32,
            );
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                value,
            );
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                value,
            );
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
