use crate::{
    piece::Color,
    square::{File, Rank, Square},
};

pub struct AlgebraicNotationConverter;

impl AlgebraicNotationConverter {
    pub fn file_and_rank_to_algebraic_notation(file: File, rank: Rank) -> String {
        let file_char = ('a' as u8 + File::from(file) as u8) as char;
        let rank_char = char::from_digit((rank as u8 + 1) as u32, 10).unwrap();
        format!("{}{}", file_char, rank_char)
    }

    pub fn algebraic_notation_to_file_and_rank(algebraic_notation: &str) -> Option<(File, Rank)> {
        let file_char = algebraic_notation.chars().next()?;
        let rank_char = algebraic_notation.chars().nth(1)?;

        let file = file_char as u8 - 'a' as u8;
        let rank = rank_char as u8 - '1' as u8;

        Some((File::from(file as usize), Rank::from(rank as usize)))
    }

    pub fn square_to_algebraic_notation(square: Square) -> String {
        Self::file_and_rank_to_algebraic_notation(square.file(), square.rank())
    }

    pub fn algebraic_notation_to_square(algebraic_notation: &str) -> Option<Square> {
        let (file, rank) = Self::algebraic_notation_to_file_and_rank(algebraic_notation)?;
        Some(Square::new(file, rank, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_and_rank_to_algebraic_notation() {
        assert_eq!(
            AlgebraicNotationConverter::file_and_rank_to_algebraic_notation(File::A, Rank::One),
            "a1"
        );

        assert_eq!(
            AlgebraicNotationConverter::file_and_rank_to_algebraic_notation(File::E, Rank::Seven),
            "e7"
        );

        assert_eq!(
            AlgebraicNotationConverter::file_and_rank_to_algebraic_notation(File::H, Rank::Eight),
            "h8"
        );
    }

    #[test]
    fn test_algebraic_notation_to_indices() {
        assert_eq!(
            AlgebraicNotationConverter::algebraic_notation_to_file_and_rank("a1"),
            Some((File::A, Rank::One))
        );

        assert_eq!(
            AlgebraicNotationConverter::algebraic_notation_to_file_and_rank("h8"),
            Some((File::H, Rank::Eight))
        );
    }
}
