use crate::{HEIGHT_PX, MINE_COUNT, WIDTH_PX};

use self::{
    game_textures::GameTextures,
    tile::{TileState, TileType},
};

pub mod game_textures;
pub mod tile;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Start,
    Playing,
    Won,
    Lost,
}

pub struct Game {
    pub tiles: Vec<Vec<tile::Tile>>,
    pub width: i32,
    pub height: i32,
    pub game_state: GameState,
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

        Game {
            tiles,
            width,
            height,
            game_state: GameState::Start,
        }
    }

    pub fn place_mines(&mut self, start_x: usize, start_y: usize) {
        let mut mines = 0;
        while mines < MINE_COUNT {
            let x = rand::random::<usize>() % self.width as usize;
            let y = rand::random::<usize>() % self.height as usize;

            if x >= start_x - 1 && x <= start_x + 1 && y >= start_y - 1 && y <= start_y + 1 {
                continue;
            }

            if self.tiles[y][x].is_bomb() {
                continue;
            }

            self.tiles[y][x].tile_type = TileType::Bomb;
            mines += 1;
        }
        self.game_state = GameState::Playing;
        self.place_numbers();
    }

    pub fn place_numbers(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y as usize][x as usize].is_bomb() {
                    continue;
                }

                let mut bombs = 0;

                for x_offset in -1 as isize..2 {
                    for y_offset in -1 as isize..2 {
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }

                        let x = x as isize + x_offset;
                        let y = y as isize + y_offset;

                        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                            if self.tiles[y as usize][x as usize].is_bomb() {
                                bombs += 1;
                            }
                        }
                    }
                }

                self.tiles[y as usize][x as usize].tile_type = tile::TileType::Empty(bombs);
            }
        }
    }

    pub fn reveal_tile(&mut self, x: usize, y: usize) {
        let tile = &mut self.tiles[y][x];

        match tile.tile_state {
            TileState::Unrevealed => match tile.tile_type {
                TileType::Bomb => {
                    tile.tile_state = TileState::Exploded;
                    self.reveal_all();
                    self.game_state = GameState::Lost;
                    return;
                }
                TileType::Empty(0) => {
                    tile.reveal();
                    for x_offset in -1 as isize..2 {
                        for y_offset in -1 as isize..2 {
                            if x_offset == 0 && y_offset == 0 {
                                continue;
                            }

                            let x = x as isize + x_offset;
                            let y = y as isize + y_offset;

                            if x >= 0
                                && x < self.width as isize
                                && y >= 0
                                && y < self.height as isize
                            {
                                self.reveal_tile(x as usize, y as usize);
                            }
                        }
                    }
                }
                _ => {
                    tile.reveal();
                    return;
                }
            },
            TileState::Revealed => {
                let bomb_count = match tile.tile_type {
                    TileType::Empty(bombs) => match bombs {
                        0 => return,
                        _ => bombs,
                    },
                    _ => return,
                };

                let mut flags = 0;
                for x_offset in -1 as isize..2 {
                    for y_offset in -1 as isize..2 {
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }

                        let x = x as isize + x_offset;
                        let y = y as isize + y_offset;

                        if x >= 0
                            && x < self.width as isize
                            && y >= 0
                            && y < self.height as isize
                            && self.tiles[y as usize][x as usize].tile_state == TileState::Flagged
                        {
                            flags += 1;
                        }
                    }
                }

                if flags == bomb_count {
                    for x_offset in -1 as isize..2 {
                        for y_offset in -1 as isize..2 {
                            if x_offset == 0 && y_offset == 0 {
                                continue;
                            }

                            let x = x as isize + x_offset;
                            let y = y as isize + y_offset;

                            if x >= 0
                                && x < self.width as isize
                                && y >= 0
                                && y < self.height as isize
                            {
                                let tile = &mut self.tiles[y as usize][x as usize];
                                if tile.tile_state == TileState::Unrevealed {
                                    self.reveal_tile(x as usize, y as usize);
                                }
                            }
                        }
                    }
                }   
            }
            _ => return,
        }
        if self.is_won() {
            self.reveal_all();
            self.game_state = GameState::Won;
        }
    }

    pub fn is_won(&self) -> bool {
        for row in &self.tiles {
            for tile in row {
                if tile.tile_type != TileType::Bomb && tile.tile_state != TileState::Revealed {
                    return false;
                }
            }
        }
        true
    }

    pub fn flag_tile(&mut self, x: usize, y: usize) {
        self.tiles[y][x].toggle_flag();
    }

    pub fn reveal_all(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                match tile.tile_state {
                    TileState::Unrevealed => tile.reveal(),
                    TileState::Exploded => (),
                    TileState::Flagged => (),
                    TileState::Revealed => (),
                }
            }
        }
    }

    pub fn left_click(&mut self, x_px: i32, y_px: i32) {
        let (x, y) = tile_position(x_px, y_px, self.width, self.height);

        if x >= self.width as usize || y >= self.height as usize {
            return;
        }

        if self.game_state == GameState::Start {
            self.place_mines(x, y);
        }

        self.reveal_tile(x, y);
    }

    pub fn right_click(&mut self, x_px: i32, y_px: i32) {
        let (x, y) = tile_position(x_px, y_px, self.width, self.height);

        if x >= self.width as usize || y >= self.height as usize {
            return;
        }

        self.flag_tile(x, y);
    }

    pub fn draw(&self, textures: &mut GameTextures) {
        for row in &self.tiles {
            for tile in row {
                tile.draw(textures);
            }
        }
    }
}

fn tile_position(x_px: i32, y_px: i32, width_tiles: i32, heigh_tiles: i32) -> (usize, usize) {
    let x = x_px as f32 / (WIDTH_PX as f32 / heigh_tiles as f32);
    let y = y_px as f32 / (HEIGHT_PX as f32 / width_tiles as f32);
    let y = heigh_tiles as f32 - y;

    (x as usize, y as usize)
}
