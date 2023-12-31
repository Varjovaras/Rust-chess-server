use core::panic;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Default, PartialOrd, Serialize, Deserialize)]

pub enum File {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn as_usize(&self) -> usize {
        match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }

    pub fn _as_str(&self) -> &'static str {
        match self {
            File::A => "A",
            File::B => "B",
            File::C => "C",
            File::D => "D",
            File::E => "E",
            File::F => "F",
            File::G => "G",
            File::H => "H",
        }
    }

    pub fn _from_str_slice(s: &str) -> File {
        let str = s.to_uppercase();
        let file = &str[..];
        match file {
            "A" => File::A,
            "B" => File::B,
            "C" => File::C,
            "D" => File::D,
            "E" => File::E,
            "F" => File::F,
            "G" => File::G,
            "H" => File::H,
            _ => panic!("Invalid file_str"),
        }
    }

    pub fn get_files() -> [File; 8] {
        [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ]
    }
}

impl From<u8> for File {
    fn from(i: u8) -> File {
        match i {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file!"),
        }
    }
}

impl From<&str> for File {
    fn from(s: &str) -> File {
        let str = s.to_uppercase();
        let file = &str[..];
        match file {
            "A" => File::A,
            "B" => File::B,
            "C" => File::C,
            "D" => File::D,
            "E" => File::E,
            "F" => File::F,
            "G" => File::G,
            "H" => File::H,
            _ => panic!("Invalid file_str"),
        }
    }
}
