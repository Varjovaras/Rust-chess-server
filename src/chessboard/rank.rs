#[derive(Copy, Clone, Debug, PartialEq, Default, PartialOrd)]

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
    pub fn _as_str(&self) -> &'static str {
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

    pub fn as_usize(&self) -> usize {
        match self {
            Rank::First => 0,
            Rank::Second => 1,
            Rank::Third => 2,
            Rank::Fourth => 3,
            Rank::Fifth => 4,
            Rank::Sixth => 5,
            Rank::Seventh => 6,
            Rank::Eighth => 7,
        }
    }

    pub fn from_str(s: &str) -> Rank {
        match s {
            "1" => Rank::First,
            "2" => Rank::Second,
            "3" => Rank::Third,
            "4" => Rank::Fourth,
            "5" => Rank::Fifth,
            "6" => Rank::Sixth,
            "7" => Rank::Seventh,
            "8" => Rank::Eighth,
            _ => panic!("Invalid rank_str"),
        }
    }
}

impl From<u8> for Rank {
    fn from(value: u8) -> Rank {
        match value {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("Invalid rank"),
        }
    }
}
