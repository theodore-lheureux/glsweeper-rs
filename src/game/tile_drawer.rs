use std::time;

use crate::graphics::ssbo::SSBO;

use super::tile::{Tile, TileState, TileType};

pub struct TileDrawer {
    ssbo: SSBO,   
}

impl TileDrawer {
    pub fn new() -> Self {
        let ssbo = SSBO::new();
        TileDrawer { ssbo }
    }

    pub fn bind_ssbo(&self) {
        self.ssbo.bind();
        self.ssbo.bind_buffer_base(0);
    }

    pub fn update(&self, tiles: &Vec<Tile>) {
        let start = time::Instant::now();
        let data = tiles.iter().map(|tile| match tile.tile_state {
            TileState::Unrevealed => 0.0,
            TileState::Flagged => 1.0,
            TileState::Revealed => match tile.tile_type {
                TileType::Empty(n) => n as f32 + 2.0,
                TileType::Bomb => 11.0,
            },
            TileState::Exploded => 12.0,
            TileState::WrongFlag => 13.0,

        }).collect::<Vec<f32>>();
        self.ssbo.bind_buffer_data(&data);
        print!("SSBO update took {:?}, ", start.elapsed());
    }
}