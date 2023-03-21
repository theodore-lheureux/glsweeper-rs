use std::ptr;

use graphics::gl_wrapper::VAO;

pub mod custom_errors;
pub mod game;
pub mod graphics;
pub mod logger;

pub const DEFAULT_WIDTH: usize = 11;
pub const DEFAULT_HEIGHT: usize = 11;

pub const MAX_WIDTH: usize = 1000;
pub const MAX_HEIGHT: usize = 1000;
pub const MIN_WIDTH: usize = 1;
pub const MIN_HEIGHT: usize = 1;

pub const WIDTH_INCREMENT: usize = 5;
pub const HEIGHT_INCREMENT: usize = 5;

pub const WIDTH_PX: i32 = 1080 - 50;
pub const HEIGHT_PX: i32 = 1080 - 50;

pub fn clear_draw(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn draw_element(vao: &VAO, count: i32) {
    vao.bind();
    unsafe {
        gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_INT, ptr::null());
    }
}
