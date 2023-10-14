use crate::{
    chess::Chess,
    chessboard::{
        get_black_king, get_squares_with_black_pieces, get_squares_with_white_pieces,
        get_white_king,
    },
    piece::{Piece, PieceColor},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Checkmate(PieceColor),
    _InsufficientMaterial,
    Stalemate,
    InProgress,
}

//
// needs rework
//
pub fn insufficient_material(chess: &Chess) -> bool {
    let white_piece_squares = get_squares_with_white_pieces(&chess.board);
    let black_piece_squares = get_squares_with_black_pieces(&chess.board);

    if get_white_king(&chess.board).is_none() || get_black_king(&chess.board).is_none() {
        return true;
    }

    if white_piece_squares.len() == 1 && black_piece_squares.len() == 1 {
        return true;
    }

    let mut white_knights: u8 = 0;
    let mut white_bishops: u8 = 0;
    let mut black_knights: u8 = 0;
    let mut black_bishops: u8 = 0;

    for square in white_piece_squares {
        match square.piece {
            Piece::None => panic!("White piece square has no piece"),
            Piece::Pawn(_) => return false,
            Piece::Knight(_) => white_knights += 1,
            Piece::Bishop(_) => white_bishops += 1,
            Piece::Rook(_) => return false,
            Piece::Queen(_) => return false,
            Piece::King(_) => {}
        }
    }

    for square in black_piece_squares {
        match square.piece {
            Piece::None => panic!("Black piece square has no piece"),
            Piece::Pawn(_) => return false,
            Piece::Knight(_) => black_knights += 1,
            Piece::Bishop(_) => black_bishops += 1,
            Piece::Rook(_) => return false,
            Piece::Queen(_) => return false,
            Piece::King(_) => {}
        }
    }

    if white_knights > 1 || black_knights > 1 || white_bishops > 1 || black_bishops > 1 {
        return false;
    }

    if white_knights == 0 && black_knights == 1 && white_bishops == 1 && black_bishops == 0 {
        return true;
    }

    if white_knights == 1 && black_knights == 0 && white_bishops == 0 && black_bishops == 1 {
        return true;
    }

    if white_knights == 2 && black_knights == 0 && white_bishops == 0 && black_bishops == 0 {
        return true;
    }

    if white_knights == 0 && black_knights == 2 && white_bishops == 0 && black_bishops == 0 {
        return true;
    }

    if white_knights == 1 && black_knights == 0 && white_bishops == 0 && black_bishops == 0 {
        return true;
    }

    if white_knights == 0 && black_knights == 1 && white_bishops == 0 && black_bishops == 0 {
        return true;
    }

    if white_knights == 0 && black_knights == 0 && white_bishops == 1 && black_bishops == 0 {
        return true;
    }
    if white_knights == 0 && black_knights == 0 && white_bishops == 0 && black_bishops == 1 {
        return true;
    }

    true
}
