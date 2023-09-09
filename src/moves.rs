use crate::{chessboard::square::Square, piece::PieceColor};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn _pawn(_color: &PieceColor, _start_sq: &Square, _board: &[[Square; 8]; 8]) -> bool {
    pawn::_move_white_pawn(_color, _start_sq, _board);
    true
}

pub fn _knight(_color: &PieceColor, _start_sq: &Square, _board: &[[Square; 8]; 8]) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _bishop(_color: &PieceColor, _start_sq: &Square, _board: &[[Square; 8]; 8]) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _rook(_color: &PieceColor, _start_sq: &Square, _board: &[[Square; 8]; 8]) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _queen(_color: &PieceColor, _start_sq: &Square, _board: &[[Square; 8]; 8]) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _king(_color: &PieceColor, _start_sq: &Square, _board: &[[Square; 8]; 8]) -> bool {
    // pawn::move_pawn();
    true
}
