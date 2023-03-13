use std::ptr;

use gl::types::{GLfloat, GLsizei};
use glsweeper_rs::{
    graphics::{
        gl_wrapper::{VertexAttribute, VAO, VBO},
        shader::Shader,
        window::Window,
    },
    logger,
};

const TRIANGLE: [f32; 9] = [
    -0.5, -0.5, 0.0, // left
    0.5, -0.5, 0.0, // right
    0.0, 0.5, 0.0, // top
];

fn main() {
    logger::init();

    let mut window = Window::new(800, 600, "GL Sweeper");

    window.init_gl();

    let shader_program =
        Shader::new("shaders/vertex.glsl", "shaders/fragment.glsl");

    let vao = VAO::new();
    vao.bind();

    let vbo = VBO::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.bind_buffer_data(&TRIANGLE);

    let vertex_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    vertex_attribute.enable();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    vao.unbind();
    vbo.unbind();

    shader_program.use_program();

    while !window.should_close() {
        window.process_events();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        vao.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        window.poll_events();
        window.clear_depth();
    }
}
