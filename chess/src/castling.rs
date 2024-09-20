use serde::{Deserialize, Serialize};

use crate::chessboard::{file::File, rank::Rank, square::Square};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Castling {
    pub white: RightToCastle,
    pub black: RightToCastle,
}

impl Castling {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            white: RightToCastle::new(),
            black: RightToCastle::new(),
        }
    }

    #[must_use]
    pub const fn castling_allowed(self, start_sq: &Square, end_sq: &Square) -> bool {
        match (start_sq.rank, end_sq.file) {
            (Rank::First, File::G) => self.white.kingside,
            (Rank::First, File::C) => self.white.queenside,
            (Rank::Eighth, File::G) => self.black.kingside,
            (Rank::Eighth, File::C) => self.black.queenside,
            _ => false,
        }
    }
}

impl Default for Castling {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RightToCastle {
    pub kingside: bool,
    pub queenside: bool,
}

impl RightToCastle {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }

    pub fn remove_castling(&mut self) {
        self.kingside = false;
        self.queenside = false;
    }
}

impl Default for RightToCastle {
    fn default() -> Self {
        Self::new()
    }
}
