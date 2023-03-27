use std::time;

use crate::{
    HEIGHT_INCREMENT, MAX_HEIGHT, MAX_WIDTH, MIN_HEIGHT, MIN_WIDTH,
    WIDTH_INCREMENT,
};

use self::{
    game_textures::GameTextures,
    tile::{Tile, TileState, TileType},
};

pub mod game_textures;
pub mod tile;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Start,
    Playing(time::Instant),
    Won(time::Duration),
    Lost(time::Duration),
}

pub struct Game {
    pub tiles: Vec<Vec<tile::Tile>>,
    pub width: usize,
    pub height: usize,
    pub state: GameState,
    pub mine_count: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::new();
        for r in 0..height {
            let mut row = Vec::new();
            for c in 0..width {
                let tile = tile::Tile::new(
                    tile::TileType::Empty(0),
                    c,
                    r,
                    width,
                    height,
                );
                row.push(tile);
            }
            tiles.push(row);
        }

        Game {
            tiles,
            width,
            height,
            state: GameState::Start,
            mine_count: width * height / 5,
        }
    }

    fn place_mines(&mut self, start_x: usize, start_y: usize) {
        let mut mines = 0;
        while mines < self.mine_count {
            let (x, y) = random_coords(self.width, self.height);
            let (start_x, start_y) = (
                if start_x == 0 { 1 } else { start_x },
                if start_y == 0 { 1 } else { start_y },
            );
            let is_adjacent = x >= start_x - 1
                && x <= start_x + 1
                && y >= start_y - 1
                && y <= start_y + 1;

            if is_adjacent || self.tiles[y][x].is_bomb() {
                continue;
            }

            self.tiles[y][x].tile_type = TileType::Bomb;
            mines += 1;
        }
        self.state = GameState::Playing(time::Instant::now());
        self.place_numbers();
    }

    fn place_numbers(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y][x].is_bomb() {
                    continue;
                }
                let bombs = self
                    .get_adjacent_tiles(x, y)
                    .iter()
                    .filter(|t| t.is_bomb())
                    .count() as u8;

                self.tiles[y][x].tile_type = tile::TileType::Empty(bombs);
            }
        }
    }

    fn reveal_tile(&mut self, x: usize, y: usize) {
        let tile = &mut self.tiles[y][x];

        if tile.is_revealed() || tile.is_flagged() {
            return;
        }
        match tile.tile_type {
            TileType::Bomb => {
                tile.tile_state = TileState::Exploded;
                self.reveal_all();
                if let GameState::Playing(start_time) = self.state {
                    self.state =
                        GameState::Lost(time::Instant::now() - start_time);
                }
            }
            TileType::Empty(0) => {
                tile.reveal();
                for x_offset in -1..2 {
                    for y_offset in -1..2 {
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
            _ => tile.reveal(),
        }
    }

    fn reveal_all(&mut self) {
        self.tiles
            .iter_mut()
            .flatten()
            .for_each(|tile| match tile.tile_type {
                TileType::Bomb => {
                    if !tile.is_exploded() && !tile.is_flagged() {
                        tile.tile_state = TileState::Revealed;
                    }
                }
                TileType::Empty(_) => {
                    if tile.is_flagged() {
                        tile.tile_state = TileState::WrongFlag;
                    }
                }
            });
    }

    fn is_won(&self) -> bool {
        self.tiles
            .iter()
            .flatten()
            .all(|tile| tile.is_revealed() || tile.is_bomb())
    }

    fn check_for_win(&mut self) {
        if !self.is_won() {
            return;
        }
        if let GameState::Playing(start_time) = self.state {
            self.state = GameState::Won(time::Instant::now() - start_time);
        }
        self.tiles
            .iter_mut()
            .flatten()
            .filter(|tile| tile.is_bomb())
            .for_each(|tile| tile.tile_state = TileState::Flagged);
    }

    fn flag_tile(&mut self, x: usize, y: usize) {
        self.tiles[y][x].toggle_flag();
    }

    fn revealed_clicked(&mut self, x: usize, y: usize) {
        let tile = &mut self.tiles[y][x];

        if !tile.is_revealed() || tile.is_flagged() {
            return;
        }

        let (bomb_count, mut flags) = (
            match tile.tile_type {
                TileType::Empty(bombs) => match bombs {
                    0 => return,
                    _ => bombs,
                },
                _ => return,
            },
            0,
        );

        for tile in self.get_adjacent_tiles(x, y) {
            if tile.is_flagged() {
                flags += 1;
            }
        }

        if flags == bomb_count {
            for x_offset in -1..2 {
                for y_offset in -1..2 {
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
                        if !tile.is_revealed() {
                            self.reveal_tile(x as usize, y as usize);
                        }
                    }
                }
            }
        }
    }

    fn get_adjacent_tiles(&self, x: usize, y: usize) -> Vec<&Tile> {
        let mut tiles = Vec::new();

        for x_offset in -1..2 {
            for y_offset in -1..2 {
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
                    tiles.push(&self.tiles[y as usize][x as usize]);
                }
            }
        }

        tiles
    }

    pub fn count_flags(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|tile| tile.is_flagged())
            .count()
    }

    pub fn left_click(
        &mut self,
        x_px: f64,
        y_px: f64,
        window_width: f64,
        window_height: f64,
    ) {
        let (x, y) = tile_position(
            x_px,
            y_px,
            self.width,
            self.height,
            window_width,
            window_height,
        );

        if x >= self.width || y >= self.height {
            return;
        }

        match self.state {
            GameState::Start => {
                self.place_mines(x, y);
                self.reveal_tile(x, y);
            }
            GameState::Playing(_) => {
                self.revealed_clicked(x, y);
                self.reveal_tile(x, y);
                self.check_for_win();
            }
            GameState::Won(_) => (),
            GameState::Lost(_) => (),
        }
    }

    pub fn right_click(
        &mut self,
        x_px: f64,
        y_px: f64,
        window_width: f64,
        window_height: f64,
    ) {
        let (x, y) = tile_position(
            x_px,
            y_px,
            self.width,
            self.height,
            window_width,
            window_height,
        );

        if x >= self.width || y >= self.height {
            return;
        }

        if matches!(self.state, GameState::Playing(_) | GameState::Start) {
            self.flag_tile(x, y);
        }
    }

    pub fn space_click(
        &mut self,
        x_px: f64,
        y_px: f64,
        window_width: f64,
        window_height: f64,
    ) {
        let (x, y) = tile_position(
            x_px,
            y_px,
            self.width,
            self.height,
            window_width,
            window_height,
        );

        if x >= self.width || y >= self.height {
            return;
        }

        let tile = &mut self.tiles[y][x];

        match self.state {
            GameState::Playing(_) | GameState::Start => (),
            _ => return,
        }
        match tile.tile_state {
            TileState::Revealed => {
                self.revealed_clicked(x, y);
                self.check_for_win();
            }
            TileState::Unrevealed | TileState::Flagged => self.flag_tile(x, y),
            _ => (),
        }
    }

    pub fn increase_size(&mut self) {
        if matches!(self.state, GameState::Playing(_)) {
            return;
        }

        let width = self.width + WIDTH_INCREMENT;
        let height = self.height + HEIGHT_INCREMENT;

        if width > MAX_WIDTH || height > MAX_HEIGHT {
            return;
        }

        *self = Self::new(width, height);
    }

    pub fn decrease_size(&mut self) {
        if matches!(self.state, GameState::Playing(_)) {
            return;
        }

        let width = self.width as isize - WIDTH_INCREMENT as isize;
        let height = self.height as isize - HEIGHT_INCREMENT as isize;

        if width < MIN_WIDTH as isize || height < MIN_HEIGHT as isize {
            return;
        }

        *self = Self::new(width as usize, height as usize);
    }

    pub fn get_time_since_start(&self) -> Option<String> {
        match self.state {
            GameState::Playing(start_time) => {
                let elapsed = start_time.elapsed();
                let seconds = elapsed.as_secs();
                let millis = elapsed.subsec_millis();

                Some(format!("{}.{:03}", seconds, millis))
            }
            _ => None,
        }
    }

    pub fn draw(&self, textures: &mut GameTextures) {
        for row in &self.tiles {
            for tile in row {
                tile.draw(textures);
            }
        }
    }
}

fn tile_position(
    x_px: f64,
    y_px: f64,
    width_tiles: usize,
    height_tiles: usize,
    window_width: f64,
    window_height: f64,
) -> (usize, usize) {
    let width_tiles = width_tiles as f64;
    let height_tiles = height_tiles as f64;
    let (offset_x, offset_y) = (
        (window_width - window_height) / 2.0,
        (window_height - window_width) / 2.0,
    );

    if window_width > window_height {
        let x = x_px - offset_x;
        (
            (x / window_height * width_tiles) as usize,
            (height_tiles - y_px / window_height * height_tiles) as usize,
        )
    } else {
        let y = y_px - offset_y;
        (
            (x_px / window_width * width_tiles) as usize,
            (height_tiles - y / window_width * height_tiles) as usize,
        )
    }
}

fn random_coords(width: usize, height: usize) -> (usize, usize) {
    (
        rand::random::<usize>() % width,
        rand::random::<usize>() % height,
    )
}
