#[derive(Clone, Debug)]
pub struct Square {
    pub file: u8,
    pub row: u8,
    pub name: String,
}

impl Square {
    pub fn new(file: u8, row: u8) -> Square {
        let file_string: String = match file {
            0 => String::from("A"),
            1 => String::from("B"),
            2 => String::from("C"),
            3 => String::from("D"),
            4 => String::from("E"),
            5 => String::from("F"),
            6 => String::from("G"),
            7 => String::from("H"),
            _ => panic!("Invalid file"),
        };
        let row_string: String = match row {
            0 => String::from("1"),
            1 => String::from("2"),
            2 => String::from("3"),
            3 => String::from("4"),
            4 => String::from("5"),
            5 => String::from("6"),
            6 => String::from("7"),
            7 => String::from("8"),
            _ => panic!("Invalid row"),
        };
        Square {
            file,
            row,
            name: String::from(file_string + &row_string),
        }
    }

    fn _get_file(&self) -> u8 {
        self.file
    }

    fn _get_row(&self) -> u8 {
        self.row
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
