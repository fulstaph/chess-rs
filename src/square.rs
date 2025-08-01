use crate::piece::{Color, Piece, PiecePrinter};

pub struct Square {
    color: Color,
    piece: Option<Piece>,
    file: File,
    rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank, piece: Option<Piece>) -> Self {
        Square {
            color: Self::color_from_file_and_rank(file, rank),
            piece,
            file,
            rank,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn piece(&self) -> Option<Piece> {
        self.piece
    }

    pub fn file(&self) -> File {
        self.file
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn remove_piece(&mut self) {
        self.piece = None;
    }

    pub fn color_from_file_and_rank(file: File, rank: Rank) -> Color {
        let file = file as usize;
        let rank = rank as usize;

        if (file + rank) % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }

    pub fn to_flat_index(&self) -> usize {
        self.rank() as usize * 8 + self.file() as usize
    }

    pub fn flat_index_from_file_and_rank(file: File, rank: Rank) -> usize {
        rank as usize * 8 + file as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl From<usize> for File {
    fn from(value: usize) -> Self {
        match value {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file index"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        match value {
            0 => Rank::One,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            _ => panic!("Invalid rank index"),
        }
    }
}

pub struct SquarePrinter {
    dark_mode: bool,
}

impl SquarePrinter {
    pub fn new(dark_mode: bool) -> Self {
        SquarePrinter { dark_mode }
    }

    pub fn to_ascii(&self, square: &Square) -> String {
        match square.piece() {
            Some(piece) => PiecePrinter::new(self.dark_mode).to_ascii(&piece),
            None => match square.color {
                Color::White => {
                    if self.dark_mode {
                        "■".to_string()
                    } else {
                        "□".to_string()
                    }
                }
                Color::Black => {
                    if self.dark_mode {
                        "□".to_string()
                    } else {
                        "■".to_string()
                    }
                }
            },
        }
    }
}
