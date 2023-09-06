use super::{file::File, rank::Rank};

#[derive(Copy, Clone, Debug, Default)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Square {
        let file = File::from(file);
        let rank = Rank::from(rank);
        Square { file, rank }
    }

    pub fn square_name(&self) -> String {
        self.file.as_str().to_owned() + self.rank.as_str()
    }
}
