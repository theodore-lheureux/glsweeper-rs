use crate::{WIDTH_PX, HEIGHT_PX};

use self::game_textures::GameTextures;

pub mod tile;
pub mod game_textures;

pub enum GameState {
    Playing,
    Won,
    Lost,
}

pub struct Game {
    pub tiles: Vec<Vec<tile::Tile>>,
    pub width: i32,
    pub height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = Vec::new();
        for r in 0..height {
            let mut row = Vec::new();
            for c in 0..width {
                let tile = tile::Tile::new(tile::TileType::Empty(0), c, r, width, height);
                row.push(tile);
            }
            tiles.push(row);
        }

        // test every tile state
        // tiles[0][0].tile_state = tile::TileState::Unrevealed;
        // tiles[0][1].tile_state = tile::TileState::Revealed;
        // tiles[0][2].tile_state = tile::TileState::Revealed;
        // tiles[0][3].tile_state = tile::TileState::Revealed;
        // tiles[0][4].tile_state = tile::TileState::Revealed;
        // tiles[0][5].tile_state = tile::TileState::Revealed;
        // tiles[0][6].tile_state = tile::TileState::Revealed;
        // tiles[0][7].tile_state = tile::TileState::Revealed;
        // tiles[0][8].tile_state = tile::TileState::Revealed;
        // tiles[0][9].tile_state = tile::TileState::Revealed;
        // tiles[0][10].tile_state = tile::TileState::Revealed;
        // tiles[0][11].tile_state = tile::TileState::Flagged;
        // tiles[0][12].tile_state = tile::TileState::Exploded;

        // tiles[0][1].tile_type = tile::TileType::Bomb;
        // tiles[0][2].tile_type = tile::TileType::Empty(0);
        // tiles[0][3].tile_type = tile::TileType::Empty(1);
        // tiles[0][4].tile_type = tile::TileType::Empty(2);
        // tiles[0][5].tile_type = tile::TileType::Empty(3);
        // tiles[0][6].tile_type = tile::TileType::Empty(4);
        // tiles[0][7].tile_type = tile::TileType::Empty(5);
        // tiles[0][8].tile_type = tile::TileType::Empty(6);
        // tiles[0][9].tile_type = tile::TileType::Empty(7);
        // tiles[0][10].tile_type = tile::TileType::Empty(8);
        

        Game {
            tiles,
            width,
            height
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &tile::Tile {
        &self.tiles[y][x]
    }

    pub fn reveal_tile(&mut self, x: usize, y: usize) {
        self.tiles[y][x].reveal();

        // if self.tiles[y][x].tile_type == tile::TileType::Empty(0) {
        //     for x in -1 as isize..2 {
        //         for y in -1 as isize..2 {
        //             if x == 0 && y == 0 {
        //                 continue;
        //             }
        //             let x = self.tiles[y as usize][x as usize].x + x;
        //             let y = self.tiles[y as usize][x as usize].y + y;
        //             if x >= 0 && x < self.width && y >= 0 && y < self.height {
        //                 if self.tiles[y as usize][x as usize].tile_state == tile::TileState::Unrevealed {
        //                     self.reveal_tile(x as usize, y as usize);
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    pub fn flag_tile(&mut self, x: usize, y: usize) {
        self.tiles[y][x].toggle_flag();
    }

    pub fn draw(&self, textures: &mut GameTextures) {
        for row in &self.tiles {
            for tile in row {
                tile.draw(textures);
            }
        }
    }

    pub fn right_click(&mut self, x_px: i32, y_px: i32) {
        let x = x_px as f32 / (WIDTH_PX as f32 / self.width as f32);
        let y = y_px as f32 / (HEIGHT_PX as f32 / self.height as f32);
        let y = self.height as f32 - y;

        self.flag_tile(x as usize, y as usize);
    }

}