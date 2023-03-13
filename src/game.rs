pub mod tile;

pub enum GameState {
    Playing,
    Won,
    Lost,
}

pub struct Game {
    pub tiles: Vec<Vec<tile::Tile>>,
    pub width: i32,
    pub height: i32
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = Vec::new();
        for r in 0..height {
            let mut row = Vec::new();
            for c in 0..width {
                row.push(tile::Tile::new(tile::TileType::Empty(0), r, c, width, height));
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

    pub fn draw(&self) {
        for row in &self.tiles {
            for tile in row {
                tile.draw();
            }
        }
    }


}