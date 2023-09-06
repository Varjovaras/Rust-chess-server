use std::default;

#[derive(Copy, Clone, Debug, PartialEq, Default)]

pub enum Rank {
    #[default]
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::First => "1",
            Rank::Second => "2",
            Rank::Third => "3",
            Rank::Fourth => "4",
            Rank::Fifth => "5",
            Rank::Sixth => "6",
            Rank::Seventh => "7",
            Rank::Eighth => "8",
        }
    }
}

impl From<u8> for Rank {
    fn from(i: u8) -> Rank {
        match i {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("Invalid file!"),
        }
    }
}
