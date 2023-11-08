use serde::{Deserialize, Serialize};

use crate::{
    chess::Chess,
    chessboard::{get_squares_with_black_pieces, get_squares_with_white_pieces, square::Square},
    piece::Piece,
};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum GameState {
    WhiteVictory,
    BlackVictory,
    InsufficientMaterial,
    Stalemate,
    InProgress,
}

pub fn insufficient_material(chess: &Chess) -> bool {
    let (white_knights, white_bishops) = count_pieces(get_squares_with_white_pieces(&chess.board));
    let (black_knights, black_bishops) = count_pieces(get_squares_with_black_pieces(&chess.board));

    if white_knights > 1 || black_knights > 1 || white_bishops > 1 || black_bishops > 1 {
        return false;
    }

    matches!(
        (white_knights, white_bishops, black_knights, black_bishops),
        (0, 1, 1, 0) | (1, 0, 0, 1) | (2, 0, 0, 0) | (0, 0, 2, 0) | (1, 0, 0, 0) | (0, 0, 1, 0)
    )
}

fn count_pieces(squares: Vec<&Square>) -> (u8, u8) {
    let mut knights = 0;
    let mut bishops = 0;

    for square in squares {
        match square.piece {
            Piece::None => panic!("Piece square has no piece"),
            Piece::Pawn(_) | Piece::Rook(_) | Piece::Queen(_) => return (0, 0),
            Piece::Knight(_) => knights += 1,
            Piece::Bishop(_) => bishops += 1,
            Piece::King(_) => {}
        }
    }

    (knights, bishops)
}
