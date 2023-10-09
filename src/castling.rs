#[derive(Debug, Copy, Clone)]
pub enum Castling {
    WhiteKingSide(bool),
    WhiteQueenSide(bool),
    BlackKingSide(bool),
    BlackQueenSide(bool),
}

impl Castling {
    pub fn new() -> Vec<Castling> {
        vec![
            Castling::WhiteKingSide(true),
            Castling::WhiteQueenSide(true),
            Castling::BlackKingSide(true),
            Castling::BlackQueenSide(true),
        ]
    }

    pub fn castling_allowed(&self) -> bool {
        match self {
            Castling::WhiteKingSide(allowed) => *allowed,
            Castling::WhiteQueenSide(allowed) => *allowed,
            Castling::BlackKingSide(allowed) => *allowed,
            Castling::BlackQueenSide(allowed) => *allowed,
        }
    }
}
