use crate::piece::{Color, Piece, PieceKind, PiecePrinter};
use crate::square::{File, Rank, Square, SquarePrinter};

const BOARD_SIZE: usize = 64;
const BOARD_WIDTH: usize = 8;

pub struct Board {
    squares: Vec<Square>,
    active_color: Color,
    half_move_clock: u8,
    full_move_number: u8,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = Vec::with_capacity(BOARD_SIZE);
        for i in 0..BOARD_SIZE {
            let file = File::from(i / BOARD_WIDTH);
            let rank = Rank::from(i % BOARD_WIDTH);
            squares.push(Square::new(file, rank, None));
        }

        Board {
            squares,
            active_color: Color::White,
            half_move_clock: 0,
            full_move_number: 1,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn get_square(&self, file: File, rank: Rank) -> &Square {
        &self.squares[Square::flat_index_from_file_and_rank(file, rank)]
    }

    fn place_piece(&mut self, piece: Piece, file: File, rank: Rank) {
        // TODO: don't create a new square if it already exists?
        // like this: `square.set_piece(piece);`
        self.squares[Square::flat_index_from_file_and_rank(file, rank)] =
            Square::new(file, rank, Some(piece));
    }

    fn remove_piece(&mut self, file: File, rank: Rank) {
        self.squares[Square::flat_index_from_file_and_rank(file, rank)].remove_piece();
    }

    pub fn setup_initial_position(&mut self) {
        // set pawns
        for file in 0..8 {
            self.place_piece(
                Piece::new(PieceKind::Pawn, Color::White),
                File::from(file),
                Rank::Two,
            );
            self.place_piece(
                Piece::new(PieceKind::Pawn, Color::Black),
                File::from(file),
                Rank::Seven,
            );
        }

        // set knights
        let white_knight = Piece::new(PieceKind::Knight, Color::White);
        let black_knight = Piece::new(PieceKind::Knight, Color::Black);
        self.place_piece(white_knight, File::B, Rank::One);
        self.place_piece(white_knight, File::G, Rank::One);
        self.place_piece(black_knight, File::B, Rank::Eight);
        self.place_piece(black_knight, File::G, Rank::Eight);

        // set bishops
        let white_bishop = Piece::new(PieceKind::Bishop, Color::White);
        let black_bishop = Piece::new(PieceKind::Bishop, Color::Black);
        self.place_piece(white_bishop, File::C, Rank::One);
        self.place_piece(white_bishop, File::F, Rank::One);
        self.place_piece(black_bishop, File::C, Rank::Eight);
        self.place_piece(black_bishop, File::F, Rank::Eight);

        // set rooks
        let white_rook = Piece::new(PieceKind::Rook, Color::White);
        let black_rook = Piece::new(PieceKind::Rook, Color::Black);
        self.place_piece(white_rook, File::A, Rank::One);
        self.place_piece(white_rook, File::H, Rank::One);
        self.place_piece(black_rook, File::A, Rank::Eight);
        self.place_piece(black_rook, File::H, Rank::Eight);

        // set set queens
        let white_queen = Piece::new(PieceKind::Queen, Color::White);
        let black_queen = Piece::new(PieceKind::Queen, Color::Black);
        self.place_piece(white_queen, File::D, Rank::One);
        self.place_piece(black_queen, File::D, Rank::Eight);

        // set set kings
        let white_king = Piece::new(PieceKind::King, Color::White);
        let black_king = Piece::new(PieceKind::King, Color::Black);
        self.place_piece(white_king, File::E, Rank::One);
        self.place_piece(black_king, File::E, Rank::Eight);
    }
}

pub struct BoardPrinter {
    dark_mode: bool,
    square_printer: SquarePrinter,
    piece_printer: PiecePrinter,
}

impl BoardPrinter {
    pub fn new(dark_mode: bool) -> Self {
        BoardPrinter {
            dark_mode: dark_mode,
            square_printer: SquarePrinter::new(dark_mode),
            piece_printer: PiecePrinter::new(dark_mode),
        }
    }

    pub fn print(&self, board: &Board) {
        let pp = PiecePrinter::new(self.dark_mode);

        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let square = board.get_square(File::from(file), Rank::from(rank));
                let piece_char = match square.piece() {
                    Some(p) => pp.to_char(&p),
                    None => match square.color() {
                        Color::White => '.',
                        Color::Black => ',',
                    },
                };
                print!("{} ", piece_char);
            }
            println!();
        }
        println!("  a b c d e f g h");
    }

    pub fn print_ascii(&self, board: &Board) {
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let square = board.get_square(File::from(file), Rank::from(rank));
                print!("{} ", self.square_printer.to_ascii(square));
            }
            println!();
        }
        println!("  a b c d e f g h");
    }

    pub fn to_fen(&self, board: &Board) -> String {
        let mut fen = String::new();
        let mut empty_squares = 0;

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = board.get_square(File::from(file), Rank::from(rank));
                match square.piece() {
                    Some(p) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push(self.piece_printer.to_char(&p));
                    }
                    None => empty_squares += 1,
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            if rank != 0 {
                fen.push('/');
            }
        }

        // fen.push(' ');
        // fen.push_str(&board.active_color.to_string());
        // fen.push(' ');
        // fen.push_str(&board.castling_rights.to_string());
        // fen.push(' ');
        // fen.push_str(&board.en_passant_target.to_string());
        // fen.push(' ');
        // fen.push_str(&board.halfmove_clock.to_string());
        // fen.push(' ');
        // fen.push_str(&board.fullmove_number.to_string());

        fen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new();
        assert_eq!(board.squares.len(), 64);
    }

    #[test]
    fn test_print() {
        let board = Board::new();
        let _ = BoardPrinter::new(true).print(&board);
    }

    #[test]
    fn test_to_ascii() {
        let board = Board::new();
        let _ = BoardPrinter::new(true).print_ascii(&board);
    }

    #[test]
    fn test_initial_representation() {
        let mut board = Board::new();
        board.setup_initial_position();

        let _ = BoardPrinter::new(true).print_ascii(&board);
    }
}
