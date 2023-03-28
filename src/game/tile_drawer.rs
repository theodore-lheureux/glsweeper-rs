use std::time;

use crate::graphics::ssbo::SSBO;

use super::tile::{Tile, TileState, TileValue};

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

    pub fn update(&self, tiles: &[Tile], has_changed: bool) {
        if !has_changed {
            return;
        }

        let start = time::Instant::now();
        let data: Vec<f32> =
            tiles.iter().map(|tile| get_texture_offset(tile)).collect();

        self.ssbo.bind_buffer_data(&data);
        print!("SSBO update took {:?}, ", start.elapsed());
    }
}

fn get_texture_offset(tile: &Tile) -> f32 {
    match tile.get_state() {
        TileState::Unrevealed => 0.0,
        TileState::Flagged => 1.0,
        TileState::Revealed => match tile.get_value() {
            TileValue::Empty(n) => n as f32 + 2.0,
            TileValue::Bomb => 11.0,
        },
        TileState::Exploded => 12.0,
        TileState::WrongFlag => 13.0,
    }
}
