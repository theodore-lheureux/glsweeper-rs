use std::ptr;

use gl::types::{GLfloat, GLsizei};

use crate::graphics::gl_wrapper::{VertexAttribute, EBO, VAO, VBO};

use super::game_textures::GameTextures;

const INDICES: [u32; 6] = [
    0, 1, 2, // first triangle
    1, 2, 3, // second triangle
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Bomb,
    Empty(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    Unrevealed,
    Revealed,
    Flagged,
    Exploded,
    WrongFlag,
}

pub struct Tile {
    pub tile_type: TileType,
    pub tile_state: TileState,
    pub vao: VAO,
}

impl Tile {
    pub fn new(
        tile_type: TileType,
        x: i32,
        y: i32,
        game_width: i32,
        game_height: i32,
    ) -> Self {
        let vao =
            generate_tile_vao(x, y, game_width as f32, game_height as f32);

        Tile {
            tile_type,
            tile_state: TileState::Unrevealed,
            vao,
        }
    }

    pub fn is_bomb(&self) -> bool {
        matches!(self.tile_type, TileType::Bomb)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.tile_type, TileType::Empty(0))
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self.tile_state, TileState::Unrevealed)
    }

    pub fn is_revealed(&self) -> bool {
        matches!(self.tile_state, TileState::Revealed)
    }

    pub fn is_flagged(&self) -> bool {
        matches!(self.tile_state, TileState::Flagged)
    }

    pub fn reveal(&mut self) {
        self.tile_state = TileState::Revealed;
    }

    pub fn toggle_flag(&mut self) {
        match self.tile_state {
            TileState::Unrevealed => self.tile_state = TileState::Flagged,
            TileState::Flagged => self.tile_state = TileState::Unrevealed,
            _ => (),
        }
    }

    pub fn unflag(&mut self) {
        self.tile_state = TileState::Unrevealed;
    }

    pub fn draw(&self, textures: &mut GameTextures) {
        self.vao.bind();

        match self.tile_state {
            TileState::Unrevealed => textures.tile_unrevealed.bind(0),
            TileState::Revealed => match self.tile_type {
                TileType::Bomb => textures.mine_revealed.bind(0),
                TileType::Empty(n) => {
                    textures.tile_revealed[n as usize].bind(0)
                }
            },
            TileState::Flagged => textures.flag.bind(0),
            TileState::WrongFlag => textures.flag_wrong.bind(0),
            TileState::Exploded => textures.mine_exploded.bind(0),
        }

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

fn generate_tile_vao(x: i32, y: i32, width: f32, height: f32) -> VAO {
    let vao = VAO::new();
    vao.bind();

    let vbo = VBO::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    let x = (x as f32 / width) * 2.0 - 1.0;
    let y = (y as f32 / height) * 2.0 - 1.0;

    let tile_size = 2.0 / width;

    let tile: [f32; 16] = [
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
    ];

    vbo.bind_buffer_data(&tile);

    let ebo = EBO::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.bind_buffer_data(&INDICES);

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
    ebo.unbind();
    vao
}
