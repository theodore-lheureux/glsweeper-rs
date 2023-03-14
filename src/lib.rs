use std::ptr;

use graphics::gl_wrapper::VAO;

pub mod logger;
pub mod custom_errors;
pub mod graphics;
pub mod game;


pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 10;

pub const WIDTH_PX: i32 = 800;
pub const HEIGHT_PX: i32 = 800;

pub const MINE_COUNT: i32 = WIDTH * HEIGHT / 5;

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