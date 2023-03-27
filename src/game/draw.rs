use std::ptr;

use gl::types::{GLfloat, GLsizei};

use crate::graphics::gl_wrapper::{VertexAttribute, VAO, VBO, EBO};

pub fn generate_game_vao(width: isize, height: isize) -> VAO {
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let (x, y, width) = (x as u32, y as u32, width as u32);

            indices.extend_from_slice(&[
                (x + y * width) * 4,
                (x + y * width) * 4 + 1,
                (x + y * width) * 4 + 2,
                (x + y * width) * 4 + 1,
                (x + y * width) * 4 + 2,
                (x + y * width) * 4 + 3,
            ]);

            let x = (x as f32 / width as f32) * 2.0 - 1.0;
            let y = (y as f32 / height as f32) * 2.0 - 1.0;

            let tile_size = 2.0 / width as f32;

            vertices.extend_from_slice(&[
                x,
                y,
                0.0,
                1.0, // top left
                x + tile_size,
                y,
                1.0,
                1.0, // top right
                x,
                y + tile_size,
                0.0,
                0.0, // bottom left
                x + tile_size,
                y + tile_size,
                1.0,
                0.0, // bottom right
            ]);

        }
    }

    let vao = VAO::new();
    vao.bind();

    let vbo = VBO::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.bind_buffer_data(&vertices);

    let ebo = EBO::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.bind_buffer_data(&indices);

    let vertex_position: VertexAttribute;
    let vertex_texture: VertexAttribute;

    unsafe {
        vertex_position = VertexAttribute::new(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
    }
    vertex_position.enable();

    unsafe {
        vertex_texture = VertexAttribute::new(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<GLfloat>() as GLsizei,
            (2 * std::mem::size_of::<GLfloat>()) as *const _,
        );
    }
    vertex_texture.enable();

    vao.unbind();
    vbo.unbind();
    vao
}
