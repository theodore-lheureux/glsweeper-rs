#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
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
    pub tile_type: TileType,
    pub tile_state: TileState,
    pub x: isize,
    pub y: isize,
}

impl Tile {
    pub fn new(tile_type: TileType, x: isize, y: isize) -> Self {
        Tile {
            tile_type,
            tile_state: TileState::Unrevealed,
            x,
            y,
        }
    }

    pub fn is_bomb(&self) -> bool {
        matches!(self.tile_type, TileType::Bomb)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.tile_type, TileType::Empty(0))
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

    pub fn reveal(&mut self) {
        self.tile_state = TileState::Revealed;
    }

    pub fn toggle_flag(&mut self) {
        match self.tile_state {
            TileState::Unrevealed => self.tile_state = TileState::Flagged,
            TileState::Flagged => self.tile_state = TileState::Unrevealed,
            _ => (),
        }
    }

    pub fn unflag(&mut self) {
        self.tile_state = TileState::Unrevealed;
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Tile {
            tile_type: self.tile_type,
            tile_state: self.tile_state,
            x: self.x,
            y: self.y,
        }
    }
}
