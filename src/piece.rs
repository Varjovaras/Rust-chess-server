use crate::{
    chess::{Chess, Move},
    chessboard::square::Square,
    moves,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
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

    pub fn _legal_moves(
        &self,
        chessboard: [[Square; 8]; 8],
        file: usize,
        rank: usize,
    ) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();
        let start_sq = chessboard[file][rank];
        (0..8).for_each(|file| {
            (0..8).for_each(|rank| {
                let end_sq = chessboard[file][rank];
                if self.piece_move(&start_sq, &end_sq, &mut Chess::new()) {
                    legal_moves.push((start_sq, end_sq, *self.color()));
                }
            });
        });
        legal_moves
    }
}
