use serde::{Deserialize, Serialize};

use crate::chessboard::{file::File, rank::Rank, square::Square};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Castling {
    pub white: RightToCastle,
    pub black: RightToCastle,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct RightToCastle {
    pub king: bool,
    pub queen: bool,
}

impl Castling {
    pub const fn new() -> Self {
        Self {
            white: RightToCastle::new(),
            black: RightToCastle::new(),
        }
    }

    pub fn _castling_allowed(self, start_sq: Square, end_sq: Square) -> bool {
        match (start_sq.rank, end_sq.file) {
            (Rank::First, File::G) => self.white.king,
            (Rank::First, File::C) => self.white.queen,
            (Rank::Eighth, File::G) => self.black.king,
            (Rank::Eighth, File::C) => self.black.queen,
            _ => false,
        }
    }
}

impl RightToCastle {
    pub const fn new() -> Self {
        Self {
            king: true,
            queen: true,
        }
    }
}
