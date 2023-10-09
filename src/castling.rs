#[derive(Debug, Copy, Clone)]
pub enum Castling {
    WhiteKing(bool),
    WhiteQueen(bool),
    BlackKing(bool),
    BlackQueen(bool),
}

impl Castling {
    pub fn new() -> Vec<Castling> {
        vec![
            Castling::WhiteKing(true),
            Castling::WhiteQueen(true),
            Castling::BlackKing(true),
            Castling::BlackQueen(true),
        ]
    }

    pub fn castling_allowed(&self) -> bool {
        match self {
            Castling::WhiteKing(allowed) => *allowed,
            Castling::WhiteQueen(allowed) => *allowed,
            Castling::BlackKing(allowed) => *allowed,
            Castling::BlackQueen(allowed) => *allowed,
        }
    }
}
