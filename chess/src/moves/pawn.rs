use crate::{
    chess::Chess,
    chessboard::square::Square,
    piece::{Piece, PieceColor},
};

pub mod black;
pub mod white;

//make this into a
#[must_use]
pub fn promote(start_sq: &Square, end_sq: &Square, chess: &Chess, piece: Piece) -> Option<Piece> {
    let promoted_piece_color = match start_sq.piece {
        Piece::Pawn(PieceColor::White) => PieceColor::White,
        Piece::Pawn(PieceColor::Black) => PieceColor::Black,
        _ => return None,
    };

    if !(end_sq.rank as u8 == 7 || end_sq.rank as u8 == 0)
        || !start_sq.piece.piece_move(start_sq, end_sq, chess)
    {
        return None;
    }

    match piece {
        Piece::King(_) | Piece::Pawn(_) | Piece::None => None,
        Piece::Knight(_) => Some(Piece::Knight(promoted_piece_color)),
        Piece::Bishop(_) => Some(Piece::Bishop(promoted_piece_color)),
        Piece::Rook(_) => Some(Piece::Rook(promoted_piece_color)),
        Piece::Queen(_) => Some(Piece::Queen(promoted_piece_color)),
    }
}
