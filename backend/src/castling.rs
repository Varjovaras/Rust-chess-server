use crate::chessboard::{file::File, rank::Rank, square::Square};

#[derive(Debug, Copy, Clone)]
pub struct Castling {
    pub white_king_side_castling: bool,
    pub white_queen_side_castling: bool,
    pub black_king_side_castling: bool,
    pub black_queen_side_castling: bool,
}

impl Castling {
    pub fn new() -> Castling {
        Castling {
            white_king_side_castling: true,
            white_queen_side_castling: true,
            black_king_side_castling: true,
            black_queen_side_castling: true,
        }
    }

    pub fn _castling_allowed(&self, start_sq: &Square, end_sq: &Square) -> bool {
        match (start_sq.rank, end_sq.file) {
            (Rank::First, File::G) => self.white_king_side_castling,
            (Rank::First, File::C) => self.white_queen_side_castling,
            (Rank::Eighth, File::G) => self.black_king_side_castling,
            (Rank::Eighth, File::C) => self.black_queen_side_castling,
            _ => false,
        }
    }
}
