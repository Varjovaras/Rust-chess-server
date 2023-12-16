use std::vec;

use serde::{Deserialize, Serialize};

use crate::{
    checkmate::MoveFromCoordinates,
    chess::Chess,
    chessboard::square::Square,
    moves,
    possible_moves::{
        bishop_possible_moves, king_possible_moves, knight_possible_moves, pawn_possible_moves,
        rook_possible_moves,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum Piece {
    #[default]
    None,
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl Piece {
    pub fn piece_move(&self, start_sq: &Square, end_sq: &Square, chess: &mut Chess) -> bool {
        match self {
            Piece::None => false,
            Piece::Pawn(color) => moves::pawn(start_sq, end_sq, chess, color),
            Piece::Knight(_) => moves::knight(start_sq, end_sq),
            Piece::Bishop(_) => moves::bishop(start_sq, end_sq, chess),
            Piece::Rook(_) => moves::rook(start_sq, end_sq, chess),
            Piece::Queen(_) => moves::queen(start_sq, end_sq, chess),
            Piece::King(_) => moves::king(start_sq, end_sq, chess),
        }
    }

    pub fn color(&self) -> &PieceColor {
        match self {
            Piece::None => &PieceColor::None,
            Piece::Pawn(color) => color,
            Piece::Knight(color) => color,
            Piece::Bishop(color) => color,
            Piece::Rook(color) => color,
            Piece::Queen(color) => color,
            Piece::King(color) => color,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Piece::None => "no piece",
            Piece::Pawn(_) => "pawn",
            Piece::Knight(_) => "knight",
            Piece::Bishop(_) => "bishop",
            Piece::Rook(_) => "rook",
            Piece::Queen(_) => "queen",
            Piece::King(_) => "king",
        }
    }

    pub fn possible_moves(&self, sq: &Square) -> Vec<MoveFromCoordinates> {
        match self {
            Piece::None => vec![],
            Piece::Pawn(_) => pawn_possible_moves(sq),
            Piece::Knight(_) => knight_possible_moves(sq),
            Piece::Bishop(_) => bishop_possible_moves(sq),
            Piece::Rook(_) => rook_possible_moves(sq),
            Piece::Queen(_) => {
                let mut possible_moves = bishop_possible_moves(sq);
                possible_moves.extend(rook_possible_moves(sq));
                possible_moves
            }
            Piece::King(_) => king_possible_moves(sq),
        }
    }
}
