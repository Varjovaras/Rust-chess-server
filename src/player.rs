use crate::piece::PieceColor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player {
    color: PieceColor,
    pub in_check: bool,
    pub won: bool,
}

impl Player {
    pub fn new(color: PieceColor) -> Player {
        Player {
            color,
            won: false,
            in_check: false,
        }
    }

    // pub fn color(&self) -> &PieceColor {
    //     &self.color
    // }

    pub fn in_check(&self) -> bool {
        self.in_check
    }

    pub fn won(&self) -> bool {
        self.won
    }
}
