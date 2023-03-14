use std::path::Path;

use crate::graphics::texture::Texture;

pub struct GameTextures {
    pub tile_unrevealed: Texture,
    pub tile_revealed: Vec<Texture>,
    pub flag: Texture,
    pub mine_exploded: Texture,
    pub mine_revealed: Texture,
}

impl GameTextures {
    pub fn new() -> Self {
        GameTextures {
            tile_unrevealed: Texture::new(Path::new("textures/tile_unrevealed.png"), 0),
            tile_revealed: vec![
                Texture::new(Path::new("textures/tile_revealed_0.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_1.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_2.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_3.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_4.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_5.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_6.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_7.png"), 0),
                Texture::new(Path::new("textures/tile_revealed_8.png"), 0),
            ],
            flag: Texture::new(Path::new("textures/flag.png"), 0),
            mine_exploded: Texture::new(Path::new("textures/mine_exploded.png"), 0),
            mine_revealed: Texture::new(Path::new("textures/mine_revealed.png"), 0),
        }
    }
}

impl Default for GameTextures {
    fn default() -> Self {
        Self::new()
    }
}