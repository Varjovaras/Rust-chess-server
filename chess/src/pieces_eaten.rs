use crate::piece::{Piece, PieceColor};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        piece::{Piece, PieceColor},
        Chess,
    };

    #[test]
    fn test_new_pieces_eaten() {
        let pieces_eaten = PiecesEaten::new();
        assert!(pieces_eaten.white.is_empty());
        assert!(pieces_eaten.black.is_empty());
    }

    #[test]
    fn test_add_white_piece() {
        let mut pieces_eaten = PiecesEaten::new();
        let white_piece = Piece::Pawn(PieceColor::White);
        pieces_eaten.add_piece(white_piece);
        assert_eq!(pieces_eaten.white.len(), 1);
        assert_eq!(pieces_eaten.white[0], white_piece);
        assert!(pieces_eaten.black.is_empty());
    }

    #[test]
    fn test_add_black_piece() {
        let mut pieces_eaten = PiecesEaten::new();
        let black_piece = Piece::Pawn(PieceColor::Black);
        pieces_eaten.add_piece(black_piece);
        assert_eq!(pieces_eaten.black.len(), 1);
        assert_eq!(pieces_eaten.black[0], black_piece);
        assert!(pieces_eaten.white.is_empty());
    }

    #[test]
    fn test_add_multiple_pieces() {
        let mut pieces_eaten = PiecesEaten::new();
        let white_piece = Piece::Pawn(PieceColor::White);
        let black_piece = Piece::Pawn(PieceColor::Black);
        pieces_eaten.add_piece(white_piece);
        pieces_eaten.add_piece(black_piece);
        assert_eq!(pieces_eaten.white.len(), 1);
        assert_eq!(pieces_eaten.black.len(), 1);
        assert_eq!(pieces_eaten.white[0], white_piece);
        assert_eq!(pieces_eaten.black[0], black_piece);
        pieces_eaten.add_piece(white_piece);
        assert_eq!(pieces_eaten.white.len(), 2);
        assert_eq!(pieces_eaten.white[0], white_piece);
        assert_eq!(pieces_eaten.black[0], black_piece);
    }

    #[test]
    fn test_with_actual_chess() {
        let white_pawn = Piece::Pawn(PieceColor::White);
        let black_pawn = Piece::Pawn(PieceColor::Black);
        let black_knight = Piece::Knight(PieceColor::Black);

        let mut chess = Chess::new_starting_position();
        chess.make_move_from_str("e2", "e4");
        chess.make_move_from_str("d7", "d5");
        chess.make_move_from_str("e4", "d5");
        assert_eq!(chess.pieces_eaten.white.len(), 0);
        assert_eq!(chess.pieces_eaten.black.len(), 1);
        assert_eq!(chess.pieces_eaten.black[0], black_pawn);
        chess.make_move_from_str("b8", "c6");
        chess.make_move_from_str("d5", "c6");
        assert_eq!(chess.pieces_eaten.white.len(), 0);
        assert_eq!(chess.pieces_eaten.black.len(), 2);
        assert_eq!(chess.pieces_eaten.black[1], black_knight);
        chess.make_move_from_str("b7", "c6");
        assert_eq!(chess.pieces_eaten.white.len(), 1);
        assert_eq!(chess.pieces_eaten.black.len(), 2);
        assert_eq!(chess.pieces_eaten.white[0], white_pawn);
        assert_eq!(chess.pieces_eaten.black[1], black_knight);
    }
}
