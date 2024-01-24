use serde::{Deserialize, Serialize};

use crate::piece::PieceColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    color: PieceColor,
    pub in_check: bool,
    pub victory: bool,
}

impl Player {
    pub const fn new(color: PieceColor) -> Self {
        Self {
            color,
            victory: false,
            in_check: false,
        }
    }

    // pub fn color(&self) -> &PieceColor {
    //     &self.color
    // }

    pub const fn in_check(self) -> bool {
        self.in_check
    }

    // pub fn victory(&self) -> bool {
    //     self.victory
    // }
}
