use serde::{Deserialize, Serialize};

use crate::{castling::RightToCastle, piece::PieceColor};

/**
 * First is kingside castling, second is queenside castling
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub color: PieceColor,
    pub in_check: bool,
    pub victory: bool,
    pub castling: RightToCastle,
}

impl Player {
    #[must_use]
    pub fn new(color: PieceColor) -> Self {
        Self {
            color,
            victory: false,
            in_check: false,
            castling: RightToCastle::default(),
        }
    }

    #[must_use]
    pub const fn color(&self) -> &PieceColor {
        &self.color
    }

    #[must_use]
    pub const fn kingside_castling_allowed(self) -> bool {
        self.castling.kingside
    }

    #[must_use]
    pub const fn queenside_castling_allowed(self) -> bool {
        self.castling.queenside
    }

    #[must_use]
    pub const fn in_check(self) -> bool {
        self.in_check
    }

    pub fn castle(&mut self) {
        self.castling.remove_castling();
    }

    pub fn no_kingside_castling(&mut self) {
        self.castling.kingside = false;
    }

    pub fn no_queenside_castling(&mut self) {
        self.castling.queenside = false;
    }
}
