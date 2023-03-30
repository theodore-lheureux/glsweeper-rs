use std::ptr;

use graphics::gl_wrapper::VAO;

pub mod game;
pub mod graphics;
pub mod logger;

pub const DEFAULT_WIDTH: isize = 11;
pub const DEFAULT_HEIGHT: isize = 11;

pub const MAX_WIDTH: isize = 950;
pub const MAX_HEIGHT: isize = 950;
pub const MIN_WIDTH: isize = 1;
pub const MIN_HEIGHT: isize = 1;

pub const WIDTH_INCREMENT: isize = 5;
pub const HEIGHT_INCREMENT: isize = 5;

pub const DEFAULT_WINDOW_WIDTH: u32 = 800;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 800;

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
