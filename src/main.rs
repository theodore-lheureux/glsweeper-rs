use std::ptr;

use gl::types::{GLfloat, GLsizei};
use glsweeper_rs::{
    graphics::{
        gl_wrapper::{VertexAttribute, VAO, VBO},
        shader,
        window::Window,
    },
    logger,
};

fn main() {
    logger::init();

    let mut window = Window::new(800, 600, "GL Sweeper");

    let triangle = [
         0.0, 0.0, 0.0,
        -1.0, 0.0, 0.0,
        -0.5, 0.5, 0.0,
    ];

    window.init_gl();

    let shader_program =
        shader::Shader::new("shaders/vertex.glsl", "shaders/fragment.glsl");


    let vao = VAO::new();
    let vbo = VBO::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    vao.bind();
    vbo.bind();
    vbo.bind_buffer_data(&triangle);

    let vertex_atributes = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    vertex_atributes.enable();

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    while !window.should_close() {
        window.process_events();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.use_program();

            vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            
            window.update();
        }
    }
}
