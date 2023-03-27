use std::time;

use crate::{
    graphics::gl_wrapper::VAO, HEIGHT_INCREMENT, MAX_HEIGHT, MAX_WIDTH,
    MIN_HEIGHT, MIN_WIDTH, WIDTH_INCREMENT,
};

use self::{
    game_textures::GameTextures,
    tile::{Tile, TileState, TileType},
};

mod coordinates;
mod draw;
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
    pub tiles: Vec<tile::Tile>,
    pub width: isize,
    pub height: isize,
    pub state: GameState,
    pub mine_count: isize,
    vao: VAO,
}

impl Game {
    pub fn new(width: isize, height: isize) -> Self {
        let mut tiles = Vec::new();

        for y in 0..height {
            for x in 0..width {
                tiles.push(Tile::new(TileType::Empty(0), x, y));
            }
        }

        Game {
            tiles,
            width,
            height,
            state: GameState::Start,
            mine_count: width * height / 5,
            vao: draw::generate_game_vao(width, height),
        }
    }

    fn init(&mut self, start_x: isize, start_y: isize) {
        if self.get_tile(start_x, start_y).is_flagged() {
            return;
        }

        self.place_mines();
        self.place_numbers();
        self.state = GameState::Playing(time::Instant::now());
    }

    fn place_mines(&mut self) {
        let mut mines = 0;

        while mines < self.mine_count {
            let (x, y) = coordinates::random_coords(self.width, self.height);

            if self.get_tile(x, y).is_bomb() {
                continue;
            }

            self.get_tile_mut(x, y).tile_type = TileType::Bomb;
            mines += 1;
        }
    }

    fn place_numbers(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_tile(x, y).is_bomb() {
                    continue;
                }

                let mut bombs = 0;

                self.do_for_adjacent_tiles(x, y, |_, tile| {
                    if tile.is_bomb() {
                        bombs += 1;
                    }
                });

                self.get_tile_mut(x, y).tile_type =
                    tile::TileType::Empty(bombs);
            }
        }
    }

    fn reveal_tile(&mut self, x: isize, y: isize) {
        let tile = &mut self.get_tile_mut(x, y);

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
                self.do_for_adjacent_tiles(x, y, |game, tile| {
                    game.reveal_tile(tile.x, tile.y);
                });
            }
            _ => tile.reveal(),
        }
    }

    fn reveal_all(&mut self) {
        self.tiles.iter_mut().for_each(|tile| match tile.tile_type {
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
            .all(|tile| tile.is_revealed() || tile.is_bomb())
    }

    fn check_for_win(&mut self) {
        if !self.is_won() {
            return;
        }
        if let GameState::Playing(start_time) = self.state {
            self.state = GameState::Won(time::Instant::now() - start_time);
        }
        self.flag_all_mines();
    }
    fn flag_all_mines(&mut self) {
        self.tiles
            .iter_mut()
            .filter(|tile| tile.is_bomb())
            .for_each(|tile| tile.tile_state = TileState::Flagged);
    }

    fn flag_tile(&mut self, x: isize, y: isize) {
        self.get_tile_mut(x, y).toggle_flag();
    }

    fn revealed_clicked(&mut self, x: isize, y: isize) {
        let tile = &mut self.get_tile(x, y);

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

        self.do_for_adjacent_tiles(x, y, |_, tile| {
            if tile.is_flagged() {
                flags += 1;
            }
        });

        if flags == bomb_count {
            self.do_for_adjacent_tiles(x, y, |game, tile| {
                if !tile.is_flagged() {
                    game.reveal_tile(tile.x, tile.y);
                }
            });
        }
    }

    fn do_for_adjacent_tiles<F>(&mut self, x: isize, y: isize, mut f: F)
    where
        F: FnMut(&mut Game, Tile),
    {
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }

                let x = x + x_offset;
                let y = y + y_offset;

                if x >= 0 && x < self.width && y >= 0 && y < self.height {
                    let tile = self.get_tile(x, y).clone();

                    f(self, tile);
                }
            }
        }
    }

    pub fn count_flags(&self) -> isize {
        self.tiles.iter().filter(|tile| tile.is_flagged()).count() as isize
    }

    pub fn left_click(
        &mut self,
        x_px: f64,
        y_px: f64,
        window_width: f64,
        window_height: f64,
    ) {
        let (x, y) = coordinates::tile_position(
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
                self.init(x, y);
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
        let (x, y) = coordinates::tile_position(
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
        let (x, y) = coordinates::tile_position(
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
            GameState::Playing(_) | GameState::Start => (),
            _ => return,
        }
        match self.get_tile(x, y).tile_state {
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

        let width = self.width - WIDTH_INCREMENT;
        let height = self.height - HEIGHT_INCREMENT;

        if width < MIN_WIDTH || height < MIN_HEIGHT {
            return;
        }

        *self = Self::new(width, height);
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

    pub fn get_tile(&self, x: isize, y: isize) -> &Tile {
        &self.tiles[(y * self.width + x) as usize]
    }

    pub fn get_tile_mut(&mut self, x: isize, y: isize) -> &mut Tile {
        &mut self.tiles[(y * self.width + x) as usize]
    }

    pub fn draw(&self, textures: &mut GameTextures) {
        self.vao.bind();

        textures.tile_unrevealed.bind(0);

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                (6 * self.width * self.height) as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
