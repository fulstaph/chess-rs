#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    pub fn material_cost(&self) -> usize {
        match self {
            PieceKind::Pawn => 1,
            PieceKind::Knight => 3,
            PieceKind::Bishop => 3,
            PieceKind::Rook => 5,
            PieceKind::Queen => 9,
            PieceKind::King => 10000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Self {
        Piece { kind, color }
    }
}

pub struct PiecePrinter {
    dark_mode: bool,
}

impl PiecePrinter {
    pub fn new(dark_mode: bool) -> Self {
        PiecePrinter { dark_mode }
    }

    pub fn to_char(&self, piece: &Piece) -> char {
        match piece.kind {
            PieceKind::Pawn => match piece.color {
                Color::White => 'P',
                Color::Black => 'p',
            },
            PieceKind::Knight => match piece.color {
                Color::White => 'N',
                Color::Black => 'n',
            },
            PieceKind::Bishop => match piece.color {
                Color::White => 'B',
                Color::Black => 'b',
            },
            PieceKind::Rook => match piece.color {
                Color::White => 'R',
                Color::Black => 'r',
            },
            PieceKind::Queen => match piece.color {
                Color::White => 'Q',
                Color::Black => 'q',
            },
            PieceKind::King => match piece.color {
                Color::White => 'K',
                Color::Black => 'k',
            },
        }
    }

    pub fn to_ascii(&self, piece: &Piece) -> String {
        match piece.kind {
            PieceKind::Pawn => match piece.color {
                Color::White => {
                    if self.dark_mode {
                        "♟".to_string()
                    } else {
                        "♙".to_string()
                    }
                }
                Color::Black => {
                    if self.dark_mode {
                        "♙".to_string()
                    } else {
                        "♟".to_string()
                    }
                }
            },
            PieceKind::Knight => match piece.color {
                Color::White => {
                    if self.dark_mode {
                        "♞".to_string()
                    } else {
                        "♘".to_string()
                    }
                }
                Color::Black => {
                    if self.dark_mode {
                        "♘".to_string()
                    } else {
                        "♞".to_string()
                    }
                }
            },
            PieceKind::Bishop => match piece.color {
                Color::White => {
                    if self.dark_mode {
                        "♝".to_string()
                    } else {
                        "♗".to_string()
                    }
                }
                Color::Black => {
                    if self.dark_mode {
                        "♗".to_string()
                    } else {
                        "♝".to_string()
                    }
                }
            },
            PieceKind::Rook => match piece.color {
                Color::White => {
                    if self.dark_mode {
                        "♜".to_string()
                    } else {
                        "♖".to_string()
                    }
                }
                Color::Black => {
                    if self.dark_mode {
                        "♖".to_string()
                    } else {
                        "♜".to_string()
                    }
                }
            },
            PieceKind::Queen => match piece.color {
                Color::White => {
                    if self.dark_mode {
                        "♛".to_string()
                    } else {
                        "♕".to_string()
                    }
                }
                Color::Black => {
                    if self.dark_mode {
                        "♕".to_string()
                    } else {
                        "♛".to_string()
                    }
                }
            },
            PieceKind::King => match piece.color {
                Color::White => {
                    if self.dark_mode {
                        "♚".to_string()
                    } else {
                        "♔".to_string()
                    }
                }
                Color::Black => {
                    if self.dark_mode {
                        "♔".to_string()
                    } else {
                        "♚".to_string()
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_printer() {
        let white_pawn = Piece::new(PieceKind::Pawn, Color::White);
        let black_pawn = Piece::new(PieceKind::Pawn, Color::Black);
        let white_knight = Piece::new(PieceKind::Knight, Color::White);
        let black_knight = Piece::new(PieceKind::Knight, Color::Black);
        let white_bishop = Piece::new(PieceKind::Bishop, Color::White);
        let black_bishop = Piece::new(PieceKind::Bishop, Color::Black);
        let white_rook = Piece::new(PieceKind::Rook, Color::White);
        let black_rook = Piece::new(PieceKind::Rook, Color::Black);
        let white_queen = Piece::new(PieceKind::Queen, Color::White);
        let black_queen = Piece::new(PieceKind::Queen, Color::Black);
        let white_king = Piece::new(PieceKind::King, Color::White);
        let black_king = Piece::new(PieceKind::King, Color::Black);

        let pp_dark = PiecePrinter::new(true);
        // let pp_normal = PiecePrinter::new(false);

        assert_eq!(pp_dark.to_char(&white_pawn), 'P');
        assert_eq!(pp_dark.to_char(&black_pawn), 'p');
        assert_eq!(pp_dark.to_char(&white_knight), 'N');
        assert_eq!(pp_dark.to_char(&black_knight), 'n');
        assert_eq!(pp_dark.to_char(&white_bishop), 'B');
        assert_eq!(pp_dark.to_char(&black_bishop), 'b');
        assert_eq!(pp_dark.to_char(&white_rook), 'R');
        assert_eq!(pp_dark.to_char(&black_rook), 'r');
        assert_eq!(pp_dark.to_char(&white_queen), 'Q');
        assert_eq!(pp_dark.to_char(&black_queen), 'q');
        assert_eq!(pp_dark.to_char(&white_king), 'K');
        assert_eq!(pp_dark.to_char(&black_king), 'k');
    }
}
