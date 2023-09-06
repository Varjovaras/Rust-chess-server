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
