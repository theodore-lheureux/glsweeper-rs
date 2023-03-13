use crate::{graphics::texture::Texture, WIDTH_PX, HEIGHT_PX};

pub mod tile;

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

                let mut tile = tile::Tile::new(tile::TileType::Empty(0), c, r, width, height);

                if rand::random() {
                    tile.toggle_flag();
                } 
                row.push(tile);

            }
            tiles.push(row);
        }
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
    }

    pub fn flag_tile(&mut self, x: usize, y: usize) {
        self.tiles[y][x].toggle_flag();
    }

    pub fn draw(&self, unrevealed_tile: &mut Texture, flag: &mut Texture) {
        for row in &self.tiles {
            for tile in row {
                tile.draw(unrevealed_tile, flag);
            }
        }
    }

    pub fn right_click(&mut self, x_px: i32, y_px: i32) {
        let x = x_px / (WIDTH_PX / self.width);
        let y = y_px / (HEIGHT_PX / self.height);
        let y = self.height - y - 1;

        self.flag_tile(x as usize, y as usize);
    }

}