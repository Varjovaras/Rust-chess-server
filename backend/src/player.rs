use serde::{Deserialize, Serialize};

use crate::piece::PieceColor;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Player {
    color: PieceColor,
    pub in_check: bool,
    pub victory: bool,
}

impl Player {
    pub fn new(color: PieceColor) -> Player {
        Player {
            color,
            victory: false,
            in_check: false,
        }
    }

    // pub fn color(&self) -> &PieceColor {
    //     &self.color
    // }

    pub fn in_check(&self) -> bool {
        self.in_check
    }

    // pub fn victory(&self) -> bool {
    //     self.victory
    // }
}
