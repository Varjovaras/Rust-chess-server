use serde::{Deserialize, Serialize};

use crate::piece::{Piece, PieceColor};

use super::{file::File, rank::Rank};

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
    pub color: SquareColor,
    pub piece: Piece,
}

impl Square {
    pub fn new(file: File, rank: Rank, color: SquareColor, piece: Piece) -> Square {
        Square {
            file,
            rank,
            color,
            piece,
        }
    }

    pub fn _new_from_u8(file: u8, rank: u8, color: SquareColor, piece: Piece) -> Square {
        let file = File::from(file);
        let rank = Rank::from(rank);
        Square {
            file,
            rank,
            color,
            piece,
        }
    }

    pub fn _new_without_piece(file: u8, rank: u8) -> Square {
        let file = File::from(file);
        let rank = Rank::from(rank);
        Square {
            file,
            rank,
            color: SquareColor::default(),
            piece: Piece::default(),
        }
    }

    pub fn _square_name(&self) -> String {
        self.file._as_str().to_owned() + self.rank._as_str()
    }

    pub fn _square_color(&self) -> SquareColor {
        self.color
    }

    pub fn is_empty(&self) -> bool {
        self.piece == Piece::default()
    }

    pub fn has_piece(&self) -> bool {
        self.piece != Piece::default()
    }

    pub fn piece_name(&self) -> &str {
        match self.piece {
            Piece::None => " ",
            Piece::Pawn(_) => match self.piece.color() {
                PieceColor::White => "p",
                PieceColor::Black => "P",
                PieceColor::None => " ",
            },
            Piece::Knight(_) => match self.piece.color() {
                PieceColor::White => "n",
                PieceColor::Black => "N",
                PieceColor::None => " ",
            },
            Piece::Bishop(_) => match self.piece.color() {
                PieceColor::White => "b",
                PieceColor::Black => "B",
                PieceColor::None => " ",
            },
            Piece::Rook(_) => match self.piece.color() {
                PieceColor::White => "r",
                PieceColor::Black => "R",
                PieceColor::None => " ",
            },
            Piece::Queen(_) => match self.piece.color() {
                PieceColor::White => "q",
                PieceColor::Black => "Q",
                PieceColor::None => " ",
            },
            Piece::King(_) => match self.piece.color() {
                PieceColor::White => "k",
                PieceColor::Black => "K",
                PieceColor::None => " ",
            },
        }
    }

    // pub fn move_piece(&mut self) {
    //     match self.piece {
    //         Piece::None => todo!(),
    //         Piece::Pawn(_) => self.piece.piece_move(),
    //         Piece::Knight(_) => todo!(),
    //         Piece::Bishop(_) => todo!(),
    //         Piece::Rook(_) => todo!(),
    //         Piece::Queen(_) => todo!(),
    //         Piece::King(_) => todo!(),
    //     }
    // }
}
