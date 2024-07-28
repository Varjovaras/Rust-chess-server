use serde::{Deserialize, Serialize};

use crate::{chess::Chess, chessboard::square::Square, moves};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
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
    #[must_use] pub fn piece_move(self, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
        match self {
            Self::None => false,
            Self::Pawn(color) => moves::pawn(start_sq, end_sq, chess, color),
            Self::Knight(_) => moves::knight(start_sq, end_sq),
            Self::Bishop(_) => moves::bishop(start_sq, end_sq, chess),
            Self::Rook(_) => moves::rook(start_sq, end_sq, chess),
            Self::Queen(_) => moves::queen(start_sq, end_sq, chess),
            Self::King(_) => moves::king(start_sq, end_sq, chess),
        }
    }

    #[must_use] pub const fn color(self) -> PieceColor {
        match self {
            Self::None => PieceColor::None,
            Self::Pawn(color)
            | Self::Knight(color)
            | Self::Bishop(color)
            | Self::Rook(color)
            | Self::Queen(color)
            | Self::King(color) => color,
        }
    }

    // pub const fn name(&self) -> &str {
    //     match self {
    //         Self::None => "no piece",
    //         Self::Pawn(_) => "pawn",
    //         Self::Knight(_) => "knight",
    //         Self::Bishop(_) => "bishop",
    //         Self::Rook(_) => "rook",
    //         Self::Queen(_) => "queen",
    //         Self::King(_) => "king",
    //     }
    // }

    // pub fn possible_legal_moves(self, chess: &Chess, start_sq: Square) -> Vec<MoveFromCoordinates> {
    //     let mut moves = vec![];
    //     match self {
    //         Self::None => {}
    //         Self::Pawn(_) => moves = pawn_possible_moves(start_sq),
    //         Self::Knight(_) => moves = knight_possible_moves(start_sq),
    //         Self::Bishop(_) => moves = bishop_possible_moves(start_sq),
    //         Self::Rook(_) => moves = rook_possible_moves(start_sq),
    //         Self::Queen(_) => {
    //             moves = {
    //                 moves = bishop_possible_moves(start_sq);
    //                 moves.append(&mut rook_possible_moves(start_sq));
    //                 moves
    //             }
    //         }
    //         Self::King(_) => moves = king_possible_moves(start_sq),
    //     }

    //     moves
    //         .iter()
    //         .filter(|possible_move| {
    //             let start_sq = chess.board[possible_move.0 .0][possible_move.0 .1];
    //             let end_sq = chess.board[possible_move.1 .0][possible_move.1 .1];
    //             check_if_move_is_legal(chess, start_sq, end_sq)
    //         })
    //         .copied()
    //         .collect::<Vec<MoveFromCoordinates>>()
    // }
}
