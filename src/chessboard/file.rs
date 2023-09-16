use core::panic;

#[derive(Copy, Clone, Debug, PartialEq, Default)]

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
    pub fn to_usize(&self) -> usize {
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

    pub fn as_str(&self) -> &'static str {
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

    pub fn from_str_slice(s: &str) -> File {
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
