use crate::{
    chessboard::{rank::Rank, square::Square},
    piece::PieceColor,
};

pub fn _move_white_pawn(
    _color: &PieceColor,
    _start_sq: &Square,
    _end_sq: &Square,
    _board: &[[Square; 8]; 8],
) -> bool {
    if _start_sq.rank == Rank::Second {
        return _white_starting_sq_move(_color, _start_sq, _end_sq, _board);
    }
    true
}

pub fn _move_black_pawn(
    _color: &PieceColor,
    _start_sq: &Square,
    _end_sq: &Square,
    _board: &[[Square; 8]; 8],
) -> bool {
    true
}

fn _white_starting_sq_move(
    _color: &PieceColor,
    _start_sq: &Square,
    _end_sq: &Square,
    _board: &[[Square; 8]; 8],
) -> bool {
    true
}

fn _black_starting_sq_move(
    _color: &PieceColor,
    _start_sq: &Square,
    _end_sq: &Square,
    _board: &[[Square; 8]; 8],
) -> bool {
    true
}
