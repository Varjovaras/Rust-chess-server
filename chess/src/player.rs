use serde::{Deserialize, Serialize};

use crate::piece::PieceColor;

/**
 * First is kingside castling, second is queenside castling
 */
type Castling = (bool, bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    color: PieceColor,
    pub in_check: bool,
    pub victory: bool,
    check: Castling,
}

impl Player {
    #[must_use]
    pub const fn new(color: PieceColor) -> Self {
        Self {
            color,
            victory: false,
            in_check: false,
            check: (true, true),
        }
    }

    #[must_use]
    pub const fn color(&self) -> &PieceColor {
        &self.color
    }

    #[must_use]
    pub const fn kingside_castling_allowed(self) -> bool {
        self.check.0
    }

    #[must_use]
    pub const fn queenside_castling_allowed(self) -> bool {
        self.check.1
    }

    #[must_use]
    pub const fn in_check(self) -> bool {
        self.in_check
    }

    pub fn castle_king_side(&mut self) {
        self.check.0 = false;
    }

    pub fn castle_queen_side(&mut self) {
        self.check.1 = false;
    }

    // pub fn victory(&self) -> bool {
    //     self.victory
    // }
}
