use serde::{Deserialize, Serialize};

use crate::piece::{Piece, PieceColor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiecesEaten {
    white: Vec<Piece>,
    black: Vec<Piece>,
}

impl PiecesEaten {
    #[must_use]
    pub fn new() -> Self {
        Self {
            white: vec![],
            black: vec![],
        }
    }

    pub fn add_piece(&mut self, piece: Piece) {
        match piece.color() {
            PieceColor::White => self.white.push(piece),
            PieceColor::Black => self.black.push(piece),
            PieceColor::None => {}
        }
    }
}

impl Default for PiecesEaten {
    fn default() -> Self {
        Self::new()
    }
}
