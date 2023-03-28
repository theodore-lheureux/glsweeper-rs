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
    has_changed: Rc<RefCell<bool>>,
    first_tile_changed: Rc<RefCell<isize>>,
}

impl Tile {
    pub fn new(
        tile_value: TileValue,
        x: isize,
        y: isize,
        has_changed: Rc<RefCell<bool>>,
        first_tile_changed: Rc<RefCell<isize>>,
    ) -> Self {
        Tile {
            tile_value,
            tile_state: TileState::Unrevealed,
            x,
            y,
            has_changed,
            first_tile_changed,
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
        *self.has_changed.borrow_mut() = true;
    }

    pub fn set_value(&mut self, value: TileValue) {
        self.tile_value = value;
        *self.has_changed.borrow_mut() = true;
    }

    pub fn toggle_flag(&mut self) {
        match self.tile_state {
            TileState::Unrevealed => self.tile_state = TileState::Flagged,
            TileState::Flagged => self.tile_state = TileState::Unrevealed,
            _ => (),
        }
        *self.has_changed.borrow_mut() = true;
    }

    pub fn unflag(&mut self) {
        self.tile_state = TileState::Unrevealed;
        *self.has_changed.borrow_mut() = true;
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            tile_value: self.tile_value,
            tile_state: self.tile_state,
            x: self.x,
            y: self.y,
            has_changed: self.has_changed.clone(),
            first_tile_changed: self.first_tile_changed.clone(),
        }
    }
}
