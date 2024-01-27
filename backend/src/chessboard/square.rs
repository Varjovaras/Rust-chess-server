use serde::{Deserialize, Serialize};

use crate::{
    checkmate::{
        bishop_possible_moves, king_possible_moves, knight_possible_moves, pawn_possible_moves,
        rook_possible_moves, MoveFromCoordinates,
    },
    chess::Chess,
    piece::{Piece, PieceColor},
};

use super::{file::File, rank::Rank};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum SquareColor {
    Black,
    #[default]
    White,
}
impl SquareColor {
    const fn _as_str(&self) -> &str {
        match self {
            Self::Black => "black",
            Self::White => "white",
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
    pub color: SquareColor,
    pub piece: Piece,
}

impl Square {
    pub const fn new(file: File, rank: Rank, color: SquareColor, piece: Piece) -> Self {
        Self {
            file,
            rank,
            color,
            piece,
        }
    }

    pub fn _new_from_u8(file: u8, rank: u8, color: SquareColor, piece: Piece) -> Self {
        let file = File::try_from(file).expect("Invalid file");
        let rank = Rank::try_from(rank).expect("Invalid rank");
        Self {
            file,
            rank,
            color,
            piece,
        }
    }

    pub fn _new_without_piece(file: u8, rank: u8) -> Self {
        let file = File::try_from(file).expect("Invalid file");
        let rank = Rank::try_from(rank).expect("Invalid rank");
        Self {
            file,
            rank,
            color: SquareColor::default(),
            piece: Piece::default(),
        }
    }

    pub fn _square_name(self) -> String {
        self.file._as_str().to_owned() + self.rank._as_str()
    }

    pub const fn _square_color(self) -> SquareColor {
        self.color
    }

    pub fn is_empty(self) -> bool {
        self.piece == Piece::default()
    }

    pub fn has_piece(self) -> bool {
        self.piece != Piece::default()
    }

    pub const fn _piece_name(self) -> &'static str {
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

    pub fn _possible_legal_moves(self, chess: &Chess, start_sq: Self) -> Vec<MoveFromCoordinates> {
        let mut moves = vec![];
        match self.piece {
            Piece::None => {}
            Piece::Pawn(_) => moves = pawn_possible_moves(start_sq),
            Piece::Knight(_) => moves = knight_possible_moves(start_sq),
            Piece::Bishop(_) => moves = bishop_possible_moves(start_sq),
            Piece::Rook(_) => moves = rook_possible_moves(start_sq),
            Piece::Queen(_) => {
                moves = {
                    moves = bishop_possible_moves(start_sq);
                    moves.append(&mut rook_possible_moves(start_sq));
                    moves
                }
            }
            Piece::King(_) => moves = king_possible_moves(start_sq),
        }

        moves
            .iter()
            .filter(|possible_move| {
                let start_sq = chess.board[possible_move.0 .0][possible_move.0 .1];
                let end_sq = chess.board[possible_move.1 .0][possible_move.1 .1];
                check_if_move_is_legal(chess, start_sq, end_sq)
            })
            .copied()
            .collect::<Vec<MoveFromCoordinates>>()
    }
}

const fn check_if_move_is_legal(chess: &Chess, start_sq: Square, end_square: Square) -> bool {
    true
}
