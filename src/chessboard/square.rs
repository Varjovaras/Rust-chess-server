use crate::piece::Pieces;

use super::{file::File, rank::Rank};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum SquareColor {
    Black,
    #[default]
    White,
}
impl SquareColor {
    fn _as_str(&self) -> &str {
        match self {
            SquareColor::Black => "black",
            SquareColor::White => "white",
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
    pub color: SquareColor,
    pub piece: Pieces,
}

impl Square {
    pub fn new(file: u8, rank: u8, color: SquareColor, piece: Pieces) -> Square {
        let file = File::from(file);
        let rank = Rank::from(rank);
        Square {
            file,
            rank,
            color,
            piece,
        }
    }

    pub fn new_without_piece(file: u8, rank: u8) -> Square {
        let file = File::from(file);
        let rank = Rank::from(rank);
        Square {
            file,
            rank,
            color: SquareColor::default(),
            piece: Pieces::default(),
        }
    }

    pub fn square_name(&self) -> String {
        self.file.as_str().to_owned() + self.rank.as_str()
    }

    pub fn _square_color(&self) -> SquareColor {
        self.color
    }

    pub fn is_empty(&self) -> bool {
        if self.piece == Pieces::default() {
            true
        } else {
            false
        }
    }

    pub fn has_piece(&self) -> bool {
        if self.piece != Pieces::default() {
            true
        } else {
            false
        }
    }
}