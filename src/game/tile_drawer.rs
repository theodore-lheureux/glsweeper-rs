use std::time;

use crate::graphics::ssbo::SSBO;

use super::tile::{Tile, TileState, TileValue};

pub struct TileDrawer {
    ssbo: SSBO,
}

impl TileDrawer {
    pub fn new(tiles: &[Tile]) -> Self {
        let ssbo = SSBO::new();
        ssbo.bind();
        ssbo.bind_buffer_base(0);

        let data = tiles
            .iter()
            .map(get_texture_offset)
            .collect::<Vec<f32>>();

        ssbo.bind_buffer_data(&data);

        TileDrawer { ssbo }
    }
    pub fn update(&self, tiles: &[Tile], tiles_changed: &[isize]) {
        if tiles_changed.is_empty() {
            return;
        }

        let start = time::Instant::now();
        let (first_index_changed, last_index_changed) = tiles_changed.iter().fold(
            (tiles_changed[0], tiles_changed[0]),
            |(min, max), &index| (min.min(index), max.max(index)),
        );

        let data: Vec<f32> = tiles
            .iter()
            .skip(first_index_changed as usize)
            .take((last_index_changed - first_index_changed + 1) as usize)
            .map(get_texture_offset)
            .collect();

        self.ssbo.bind_buffer_sub_data(
            first_index_changed * std::mem::size_of::<f32>() as isize,
            &data,
        );
        println!("SSBO update took {:?}, ", start.elapsed());
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
