use crate::graphics::texture::Texture;

pub struct GameTextures {
    pub tile_unrevealed: Texture,
    pub tile_revealed: Vec<Texture>,
    pub flag: Texture,
    pub flag_wrong: Texture,
    pub mine_exploded: Texture,
    pub mine_revealed: Texture,
}

impl GameTextures {
    pub fn new() -> Self {
        GameTextures {
            tile_unrevealed: Texture::new(
                include_bytes!("../../textures/tile_unrevealed.png").to_vec(),
                0,
            ),
            tile_revealed: vec![
                Texture::new(include_bytes!("../../textures/tile_revealed_0.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_1.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_2.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_3.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_4.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_5.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_6.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_7.png").to_vec(), 0),
                Texture::new(include_bytes!("../../textures/tile_revealed_8.png").to_vec(), 0),
            ],
            flag: Texture::new(include_bytes!("../../textures/flag.png").to_vec(), 0),
            flag_wrong: Texture::new(include_bytes!("../../textures/flag_wrong.png").to_vec(), 0),
            mine_exploded: Texture::new(
                include_bytes!("../../textures/mine_exploded.png").to_vec(),
                0,
            ),
            mine_revealed: Texture::new(
                include_bytes!("../../textures/mine_revealed.png").to_vec(),
                0,
            ),
        }
    }
}

impl Default for GameTextures {
    fn default() -> Self {
        Self::new()
    }
}
