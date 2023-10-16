use crate::{
    chess::Chess,
    chessboard::square::Square,
    piece::{Piece, PieceColor},
};

pub mod black_pawn;
pub mod white_pawn;

pub fn promote(start_sq: &Square, end_sq: &Square, chess: &mut Chess) -> Option<Piece> {
    let promoted_piece_color = match start_sq.piece {
        Piece::Pawn(PieceColor::White) => PieceColor::White,
        Piece::Pawn(PieceColor::Black) => PieceColor::Black,
        _ => return None,
    };
    if start_sq.piece != Piece::Pawn(PieceColor::White)
        && start_sq.piece != Piece::Pawn(PieceColor::Black)
        && !(end_sq.rank as u8 == 7 || end_sq.rank as u8 == 0)
    {
        return None;
    }

    if !start_sq.piece.piece_move(start_sq, end_sq, chess) {
        return None;
    }

    if end_sq.rank as u8 == 7 || end_sq.rank as u8 == 0 {
        return Some(Piece::Queen(promoted_piece_color));
    }

    None
}
