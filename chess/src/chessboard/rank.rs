use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Default, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
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
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub const fn _as_str(self) -> &'static str {
        match self {
            Self::First => "1",
            Self::Second => "2",
            Self::Third => "3",
            Self::Fourth => "4",
            Self::Fifth => "5",
            Self::Sixth => "6",
            Self::Seventh => "7",
            Self::Eighth => "8",
        }
    }

    #[must_use]
    pub const fn as_usize(self) -> usize {
        match self {
            Self::First => 0,
            Self::Second => 1,
            Self::Third => 2,
            Self::Fourth => 3,
            Self::Fifth => 4,
            Self::Sixth => 5,
            Self::Seventh => 6,
            Self::Eighth => 7,
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn _from_str(s: &str) -> Self {
        match s {
            "1" => Self::First,
            "2" => Self::Second,
            "3" => Self::Third,
            "4" => Self::Fourth,
            "5" => Self::Fifth,
            "6" => Self::Sixth,
            "7" => Self::Seventh,
            "8" => Self::Eighth,
            _ => panic!("Invalid rank_str"),
        }
    }

    pub(crate) const fn get_ranks() -> [Self; 8] {
        [
            Self::First,
            Self::Second,
            Self::Third,
            Self::Fourth,
            Self::Fifth,
            Self::Sixth,
            Self::Seventh,
            Self::Eighth,
        ]
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::First),
            1 => Ok(Self::Second),
            2 => Ok(Self::Third),
            3 => Ok(Self::Fourth),
            4 => Ok(Self::Fifth),
            5 => Ok(Self::Sixth),
            6 => Ok(Self::Seventh),
            7 => Ok(Self::Eighth),
            _ => Err(()),
        }
    }
}

impl TryFrom<usize> for Rank {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::First),
            1 => Ok(Self::Second),
            2 => Ok(Self::Third),
            3 => Ok(Self::Fourth),
            4 => Ok(Self::Fifth),
            5 => Ok(Self::Sixth),
            6 => Ok(Self::Seventh),
            7 => Ok(Self::Eighth),
            _ => panic!("Invalid rank"),
        }
    }
}

impl TryFrom<&str> for Rank {
    type Error = ();

    fn try_from(s: &str) -> std::result::Result<Self, ()> {
        match s {
            "1" => Ok(Self::First),
            "2" => Ok(Self::Second),
            "3" => Ok(Self::Third),
            "4" => Ok(Self::Fourth),
            "5" => Ok(Self::Fifth),
            "6" => Ok(Self::Sixth),
            "7" => Ok(Self::Seventh),
            "8" => Ok(Self::Eighth),
            _ => panic!("Invalid rank_str"),
        }
    }
}
