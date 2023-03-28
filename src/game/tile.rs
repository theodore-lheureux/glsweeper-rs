use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileValue {
    Bomb,
    Empty(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    Unrevealed,
    Revealed,
    Flagged,
    Exploded,
    WrongFlag,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    pub x: isize,
    pub y: isize,
    tile_value: TileValue,
    tile_state: TileState,
    tiles_changed: Rc<RefCell<Vec<isize>>>,
    game_width: isize,
}

impl Tile {
    pub fn new(
        tile_value: TileValue,
        x: isize,
        y: isize,
        tiles_changed: Rc<RefCell<Vec<isize>>>,
        game_width: isize,
    ) -> Self {
        Tile {
            tile_value,
            tile_state: TileState::Unrevealed,
            x,
            y,
            tiles_changed,
            game_width,
        }
    }

    pub fn is_bomb(&self) -> bool {
        matches!(self.tile_value, TileValue::Bomb)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.tile_value, TileValue::Empty(0))
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self.tile_state, TileState::Unrevealed)
    }

    pub fn is_revealed(&self) -> bool {
        matches!(self.tile_state, TileState::Revealed)
    }

    pub fn is_flagged(&self) -> bool {
        matches!(self.tile_state, TileState::Flagged)
    }

    pub fn is_exploded(&self) -> bool {
        matches!(self.tile_state, TileState::Exploded)
    }

    pub fn get_state(&self) -> TileState {
        self.tile_state
    }

    pub fn get_value(&self) -> TileValue {
        self.tile_value
    }

    pub fn set_state(&mut self, state: TileState) {
        self.tile_state = state;
        self.changed();
    }

    pub fn set_value(&mut self, value: TileValue) {
        self.tile_value = value;
        self.changed();
    }

    pub fn toggle_flag(&mut self) {
        match self.tile_state {
            TileState::Unrevealed => self.tile_state = TileState::Flagged,
            TileState::Flagged => self.tile_state = TileState::Unrevealed,
            _ => (),
        }
        self.changed();
    }

    pub fn unflag(&mut self) {
        self.tile_state = TileState::Unrevealed;
        self.changed();
    }

    fn changed(&mut self) {
        self.tiles_changed
            .borrow_mut()
            .push(self.x + self.y * self.game_width);
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            tile_value: self.tile_value,
            tile_state: self.tile_state,
            x: self.x,
            y: self.y,
            tiles_changed: self.tiles_changed.clone(),
            game_width: self.game_width,
        }
    }
}
