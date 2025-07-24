pub mod error;
pub mod file;
pub mod player;
pub mod rank;
pub mod ray;
pub mod square;

use ray::Ray;
use serde::{Deserialize, Serialize, Serializer, Deserializer};

pub use crate::file::File;
pub use crate::rank::Rank;
pub use crate::square::*;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not};

/// A good old-fashioned bitboard
/// You *do* have access to the actual value, but you are probably better off
/// using the implemented operators to work with this object.
///
/// ```
/// use chess::{BitBoard, Square};
///
/// let bb = BitBoard(7); // lower-left 3 squares
///
/// let mut count = 0;
///
/// // Iterate over each square in the bitboard
/// for _ in bb {
///     count += 1;
/// }
///
/// assert_eq!(count, 3);
/// ```
///
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u128);

// Impl BitAnd
impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

// Impl BitOr
impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

// Impl BitXor

impl BitXor for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

// Impl BitAndAssign

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: &BitBoard) {
        self.0 &= other.0;
    }
}

// Impl BitOrAssign
impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: &BitBoard) {
        self.0 |= other.0;
    }
}

// Impl BitXor Assign
impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: &BitBoard) {
        self.0 ^= other.0;
    }
}

// Impl Mul
impl Mul for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

// Impl Not
impl Not for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl fmt::Display for BitBoard {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (ix, square) in BitBoard::display_squares().enumerate() {
            if (self & BitBoard::from(square)).is_empty() {
                write!(f, ". ")?;
            } else {
                write!(f, "X ")?;
            }
            if ix % 8 == 7 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl BitBoard {
    pub const FAR_NORTH: Self = Self(0xff000000000000000000000000000000);
    pub const FAR_SOUTH: Self = Self(0x000000000000000000000000000000ff);
    pub const FAR_EAST: Self = Self(0x80808080808080808080808080808080);
    pub const FAR_WEST: Self = Self(0x01010101010101010101010101010101);

    pub const STANDARD_MASK: Self = Self(0x0000000000000000ffffffffffffffff);
    pub const FULL: Self = Self(0xffffffffffffffffffffffffffffffff);
    pub const EMPTY: Self = Self(0);

    /// Construct a new bitboard from a u64
    #[inline]
    pub fn new(b: u128) -> BitBoard {
        BitBoard(b)
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    #[inline]
    pub fn set(rank: Rank, file: File) -> BitBoard {
        BitBoard::from_square(Square::make_square(rank, file))
    }

    /// See [u128::rotate_left]
    #[inline]
    pub fn rotate_left(self, n: u32) -> BitBoard {
        BitBoard(self.0.rotate_left(n))
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    #[inline]
    pub fn from_square(sq: Square) -> BitBoard {
        BitBoard(1u128 << sq.to_int())
    }

    /// Convert an `Option<Square>` to an `Option<BitBoard>`
    #[inline]
    pub fn from_maybe_square(sq: Option<Square>) -> Option<BitBoard> {
        sq.map(|s| BitBoard::from_square(s))
    }

    /// Convert a `BitBoard` to a `Square`.  This grabs the least-significant `Square`
    #[inline]
    pub fn to_square(&self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    /// Count the number of `Squares` set in this `BitBoard`
    #[inline]
    pub fn len(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn popcnt(&self) -> u32 {
        self.len()
    }

    /// Reverse this `BitBoard`.  Look at it from the opponents perspective.
    #[inline]
    pub fn reverse_colors(&self) -> BitBoard {
        BitBoard(self.0.swap_bytes())
    }

    /// Convert this `BitBoard` to a `usize` (for table lookups)
    #[inline]
    pub fn to_size(&self, rightshift: u8) -> usize {
        (self.0 >> rightshift) as usize
    }

    #[inline]
    pub fn shift_to(self, square: Square) -> Self {
        let (file, rank) = square.file_rank();
        let shifted = Ray::East.shift_n(self, file.to_index());
        Ray::North.shift_n(shifted, rank.to_index())
    }

    #[inline]
    pub fn contains(self, square: Square) -> bool {
        !(self & Self::from(square)).is_empty()
    }

    pub fn display_squares() -> impl Iterator<Item = Square> {
        (0..8).rev().flat_map(|rank| {
            (0..8).map(move |file| {
                Square::make_square(Rank::from_index(rank), File::from_index(file))
            })
        })
    }

    pub fn from_standard(board: &str) -> Self {
        let mut pos = 1u128 << (63 - 7);
        let mut bits = 0u128;
        let mut ix = 0;
        for c in board.chars() {
            let shift = match c {
                '.' => 1,
                'K' | 'Q' | 'R' | 'B' | 'N' | 'T' | 'X' | 'x' => {
                    bits += pos;
                    1
                }
                _ => 0,
            };
            pos = pos << shift;
            ix += shift;
            if shift > 0 {
                if ix > 0 && ix % 8 == 0 {
                    pos = pos >> 16;
                }
            }
        }

        BitBoard(bits)
    }
}

/// For the `BitBoard`, iterate over every `Square` set.
impl Iterator for BitBoard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let result = self.to_square();
            *self ^= BitBoard::from_square(result);
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.popcnt() as usize, Some(self.popcnt() as usize))
    }
}

impl ExactSizeIterator for BitBoard {}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        Self::from_square(value)
    }
}

/// For the `BitBoard`, iterate over every `Square` set.
impl FromIterator<Square> for BitBoard {
    fn from_iter<T: IntoIterator<Item = Square>>(iter: T) -> Self {
        iter.into_iter().fold(Self::EMPTY, |board, square| {
            board | BitBoard::from_square(square)
        })
    }
}

impl Serialize for BitBoard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let squares: Vec<Square> = self.clone().collect();
        let mut seq = serializer.serialize_seq(Some(squares.len()))?;
        for square in squares {
            seq.serialize_element(&square)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for BitBoard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let squares: Vec<Square> = Vec::deserialize(deserializer)?;
        Ok(squares.into_iter().collect::<BitBoard>())
    }
}

#[cfg(test)]
pub mod test {
    use file::ALL_FILES;
    use rank::ALL_RANKS;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_strings() {
        let a1 = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            x . . . . . . .
        "#,
        );
        assert_eq!(
            a1.to_string(),
            BitBoard::from_square(Square::A1).to_string()
        );

        let h8 = BitBoard::from_standard(
            r#"
            . . . . . . . x
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );
        assert_eq!(
            h8.to_string(),
            BitBoard::from_square(Square::H8).to_string()
        );
    }

    #[test]
    fn test_square_round_trip() {
        for rank in ALL_RANKS {
            for file in ALL_FILES {
                let square = Square::make_square(rank, file);
                let mut board = BitBoard::from_square(square);
                assert_eq!(board.next(), Some(square));
                assert_eq!(board.next(), None);
            }
        }
    }

    #[test]
    fn test_shifting() {
        let test = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . x x . . . . .
        "#,
        );

        let expected = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . x x . . .
        "#,
        );

        assert_eq!(test.shift_to(Square::A1).to_string(), test.to_string());
        assert_eq!(test.shift_to(Square::C1).to_string(), expected.to_string());

        let expected = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . x x . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );
        assert_eq!(test.shift_to(Square::C5).to_string(), expected.to_string());

        let expected = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . x
        "#,
        );

        assert_eq!(test.shift_to(Square::G1).to_string(), expected.to_string());
        assert_eq!(
            test.shift_to(Square::H1).to_string(),
            BitBoard::EMPTY.to_string()
        );
    }

    #[test]
    fn test_square_serialization() {
        let sq = Square::make_square(Rank::First, File::A);
        let serialized = serde_json::to_string(&sq).unwrap();
        assert_eq!(serialized, "\"a1\"");

        let deserialized: Square = serde_json::from_str("\"a1\"").unwrap();
        assert_eq!(deserialized, sq);

        let sq2 = Square::make_square(Rank::Eighth, File::H);
        let serialized2 = serde_json::to_string(&sq2).unwrap();
        assert_eq!(serialized2, "\"h8\"");

        let deserialized2: Square = serde_json::from_str("\"h8\"").unwrap();
        assert_eq!(deserialized2, sq2);
    }

    #[test]
    fn test_bitboard_serialization() {
        // Test empty bitboard
        let empty = BitBoard::EMPTY;
        let serialized = serde_json::to_string(&empty).unwrap();
        assert_eq!(serialized, "[]");

        let deserialized: BitBoard = serde_json::from_str("[]").unwrap();
        assert_eq!(deserialized, empty);

        // Test single square bitboard
        let single = BitBoard::from_square(Square::make_square(Rank::First, File::A));
        let serialized = serde_json::to_string(&single).unwrap();
        assert_eq!(serialized, "[\"a1\"]");

        let deserialized: BitBoard = serde_json::from_str("[\"a1\"]").unwrap();
        assert_eq!(deserialized, single);

        // Test multiple squares bitboard
        let multiple = BitBoard::from_square(Square::A1) | BitBoard::from_square(Square::C3) | BitBoard::from_square(Square::H8);
        let serialized = serde_json::to_string(&multiple).unwrap();
        // The order might vary, so we check that it contains all squares
        let squares: Vec<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(squares.len(), 3);
        assert!(squares.contains(&"a1".to_string()));
        assert!(squares.contains(&"c3".to_string()));
        assert!(squares.contains(&"h8".to_string()));

        let deserialized: BitBoard = serde_json::from_str("[\"a1\",\"c3\",\"h8\"]").unwrap();
        assert_eq!(deserialized.contains(Square::A1), true);
        assert_eq!(deserialized.contains(Square::C3), true);
        assert_eq!(deserialized.contains(Square::H8), true);
        assert_eq!(deserialized.len(), 3);
    }
}
