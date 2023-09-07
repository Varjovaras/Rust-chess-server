use super::{file::File, rank::Rank};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum SquareColor {
    Black,
    #[default]
    White,
}
impl SquareColor {
    fn as_str(&self) -> &str {
        match self {
            SquareColor::Black => "black",
            SquareColor::White => "white",
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
    pub color: SquareColor,
}

impl Square {
    pub fn new(file: u8, rank: u8, color: SquareColor) -> Square {
        let file = File::from(file);
        let rank = Rank::from(rank);
        Square { file, rank, color }
    }

    pub fn square_name(&self) -> String {
        self.file.as_str().to_owned() + self.rank.as_str()
    }

    pub fn square_color(&self) -> SquareColor {
        self.color
    }
}
