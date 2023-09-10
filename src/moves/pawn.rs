use crate::{
    chess::Chess,
    chessboard::{rank::Rank, square::Square},
    piece::PieceColor,
};

pub fn move_white_pawn(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    chess: &Chess,
) -> bool {
    if start_sq.rank == Rank::Second {
        white_starting_sq_move(color, start_sq, end_sq, chess)
    } else {
        one_square_forward()
    }
}

pub fn move_black_pawn(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    chess: &Chess,
) -> bool {
    true
}

fn white_starting_sq_move(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    chess: &Chess,
) -> bool {
    if start_sq.file != end_sq.file {
        return en_passant();
    }
    let rank = end_sq.rank;
    // if

    true
}

fn en_passant() -> bool {
    true
}

fn one_square_forward() -> bool {
    true
}

fn _black_starting_sq_move(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    chess: &Chess,
) -> bool {
    true
}
