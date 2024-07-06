use crate::BitBoard;

#[derive(Debug, Clone, Copy)]
pub enum Ray {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Ray {
    pub const ALL_RAYS: [Ray; 8] = [
        Ray::North,
        Ray::NorthEast,
        Ray::East,
        Ray::SouthEast,
        Ray::South,
        Ray::SouthWest,
        Ray::West,
        Ray::NorthWest,
    ];

    pub const STRAIGHTS: [Ray; 4] = [Ray::North, Ray::East, Ray::South, Ray::West];

    pub const DIAGONALS: [Ray; 4] = [
        Ray::NorthEast,
        Ray::SouthEast,
        Ray::SouthWest,
        Ray::NorthWest,
    ];

    pub fn rotation(&self) -> u32 {
        match self {
            Ray::North => 8,
            Ray::NorthEast => 9,
            Ray::East => 1,
            Ray::SouthEast => 128 - 7,
            Ray::South => 128 - 8,
            Ray::SouthWest => 128 - 9,
            Ray::West => 128 - 1,
            Ray::NorthWest => 7,
        }
    }

    pub fn wraps(&self) -> BitBoard {
        match self {
            Ray::North => BitBoard::FAR_NORTH,
            Ray::NorthEast => BitBoard::FAR_NORTH | BitBoard::FAR_EAST,
            Ray::East => BitBoard::FAR_EAST,
            Ray::SouthEast => BitBoard::FAR_SOUTH | BitBoard::FAR_EAST,
            Ray::South => BitBoard::FAR_SOUTH,
            Ray::SouthWest => BitBoard::FAR_SOUTH | BitBoard::FAR_WEST,
            Ray::West => BitBoard::FAR_WEST,
            Ray::NorthWest => BitBoard::FAR_NORTH | BitBoard::FAR_WEST,
        }
    }

    #[inline]
    pub fn step_empty(self, board: BitBoard) -> BitBoard {
        self.step(board, !BitBoard::EMPTY, !BitBoard::EMPTY)
    }

    #[inline]
    pub fn shift(self, board: BitBoard) -> BitBoard {
        (board & !self.wraps()).rotate_left(self.rotation())
    }

    #[inline]
    pub fn shift_n(self, board: BitBoard, n: usize) -> BitBoard {
        let mut result = board;
        for _ in 0..n {
            result = self.shift(result);
        }
        result
    }

    #[inline]
    pub fn step_empty_n(self, board: BitBoard, n: u8) -> BitBoard {
        let mut result = board;
        for _ in 0..n {
            result = self.step(board, !BitBoard::EMPTY, !BitBoard::EMPTY)
        }
        result
    }

    #[inline]
    fn step_mask(self, board: &mut BitBoard, pre: BitBoard, post: BitBoard) -> BitBoard {
        *board = (*board & pre).rotate_left(self.rotation()) & post;
        *board
    }

    pub fn dumb7(self, mut sliders: BitBoard, empty: BitBoard) -> BitBoard {
        let mut flood = sliders.clone();
        let nowrap = !self.wraps();
        flood |= self.step_mask(&mut sliders, nowrap, empty);
        flood |= self.step_mask(&mut sliders, nowrap, empty);
        flood |= self.step_mask(&mut sliders, nowrap, empty);
        flood |= self.step_mask(&mut sliders, nowrap, empty);
        flood |= self.step_mask(&mut sliders, nowrap, empty);
        flood |= self.step_mask(&mut sliders, nowrap, empty);
        flood | self.step_mask(&mut sliders, nowrap, empty)
    }

    pub fn slides(self, sliders: BitBoard, empty: BitBoard, targets: BitBoard) -> BitBoard {
        let mut quiet = self.dumb7(sliders, empty);
        quiet | self.step_mask(&mut quiet, !self.wraps(), targets)
    }

    pub fn step(self, mut steppers: BitBoard, empty: BitBoard, targets: BitBoard) -> BitBoard {
        steppers | self.step_mask(&mut steppers, !self.wraps(), empty | targets)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{Rank, Square};
    use pretty_assertions::assert_eq;

    fn assert_step(from: Square, ray: Ray, to: Square) {
        let start = BitBoard::from_square(from);
        let actual = ray.step_empty(start);
        let expected = start | BitBoard::from_square(to);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    fn assert_no_step(from: Square, ray: Ray) {
        let start = BitBoard::from_square(from);
        let actual = ray.step_empty(start);
        let expected = BitBoard::from_square(from);

        assert_eq!(actual.to_string(), expected.to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_single_step() {
        assert_step(Square::B2, Ray::North, Square::B3);
        assert_step(Square::B2, Ray::NorthEast, Square::C3);
        assert_step(Square::B2, Ray::East, Square::C2);
        assert_step(Square::B2, Ray::SouthEast, Square::C1);
        assert_step(Square::B2, Ray::South, Square::B1);
        assert_step(Square::B2, Ray::SouthWest, Square::A1);
        assert_step(Square::B2, Ray::West, Square::A2);
        assert_step(Square::B2, Ray::NorthWest, Square::A3);
    }

    #[test]
    fn test_single_step_wrap() {
        assert_no_step(
            Square::make_square(Rank::Sixteenth, crate::File::A),
            Ray::NorthWest,
        );
        assert_no_step(
            Square::make_square(Rank::Sixteenth, crate::File::E),
            Ray::North,
        );
        assert_no_step(
            Square::make_square(Rank::Sixteenth, crate::File::H),
            Ray::NorthEast,
        );
        assert_no_step(Square::H3, Ray::East);
        assert_no_step(Square::H1, Ray::SouthEast);
        assert_no_step(Square::G1, Ray::South);
        assert_no_step(Square::A1, Ray::SouthWest);
        assert_no_step(Square::A7, Ray::West);
    }

    #[test]
    fn test_king_raycast() {
        let start = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . K . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let occupied = BitBoard::from_standard(
            r#"
            . . . x . . . .
            . . . x . . x .
            . . . . . . . .
            . . x . . . . .
            x x x K . . x .
            . . . . . . . .
            . . . . . x . .
            x . . . . . . .
        "#,
        );

        let targets = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . x . . . . .
            . x . . . . x .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let blocked = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . x x x . . .
            . . . K x . . .
            . . x x x . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let mut filled = start.clone();
        for ray in Ray::ALL_RAYS {
            filled |= ray.step(start, !occupied, targets);
        }

        assert_eq!(filled.to_string(), blocked.to_string());
    }

    #[test]
    fn test_queen_raycast() {
        let start = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . x . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let simple = BitBoard::from_standard(
            r#"
            . . . x . . . x
            x . . x . . x .
            . x . x . x . .
            . . x x x . . .
            x x x x x x x x
            . . x x x . . .
            . x . x . x . .
            x . . x . . x .
        "#,
        );

        let mut filled = start.clone();
        for ray in Ray::ALL_RAYS {
            filled |= ray.dumb7(start, !BitBoard::EMPTY);
        }

        assert_eq!(filled.to_string(), simple.to_string());

        let occupied = BitBoard::from_standard(
            r#"
            . . . x . . . .
            . . . x . . x .
            . . . . . . . .
            . . x . . . . .
            x x x Q . . x .
            . . . . . . . .
            . . . . . x . .
            x . . . . . . .
        "#,
        );

        let targets = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . x . . . . .
            . x . . . . x .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let blocked = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . x . x . .
            . . x x x . . .
            . . . Q x x x .
            . . x x x . . .
            . x . x . . . .
            . . . x . . . .
        "#,
        );

        let mut filled = start.clone();
        for ray in Ray::ALL_RAYS {
            filled |= ray.slides(start, !occupied, targets);
        }

        assert_eq!(filled.to_string(), blocked.to_string());
    }

    #[test]
    fn test_rook_raycast() {
        let start = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . R . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let simple = BitBoard::from_standard(
            r#"
            . . . x . . . .
            . . . x . . . .
            . . . x . . . .
            . . . x . . . .
            x x x R x x x x
            . . . x . . . .
            . . . x . . . .
            . . . x . . . .
        "#,
        );

        let mut filled = start.clone();
        for ray in Ray::STRAIGHTS {
            filled |= ray.dumb7(start, !BitBoard::EMPTY);
        }

        assert_eq!(filled.to_string(), simple.to_string());

        let occupied = BitBoard::from_standard(
            r#"
            . . . x . . . .
            . . . x . . x .
            . . . . . . . .
            . . x . . . . .
            x x x R . . x .
            . . . . . . . .
            . . . . . x . .
            x . . . . . . .
        "#,
        );

        let targets = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . x . . . . .
            . x . . . . x .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let blocked = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . x . . . .
            . . . x . . . .
            . . . R x x x .
            . . . x . . . .
            . . . x . . . .
            . . . x . . . .
        "#,
        );

        let mut filled = start.clone();
        for ray in Ray::STRAIGHTS {
            filled |= ray.slides(start, !occupied, targets);
        }

        assert_eq!(filled.to_string(), blocked.to_string());
    }

    #[test]
    fn test_bishop_raycast() {
        let start = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . . B . . . .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let simple = BitBoard::from_standard(
            r#"
            . . . . . . . x
            x . . . . . x .
            . x . . . x . .
            . . x . x . . .
            . . . B . . . .
            . . x . x . . .
            . x . . . x . .
            x . . . . . x .
        "#,
        );

        let mut filled = start.clone();
        for ray in Ray::DIAGONALS {
            filled |= ray.dumb7(start, !BitBoard::EMPTY);
        }

        assert_eq!(filled.to_string(), simple.to_string());

        let occupied = BitBoard::from_standard(
            r#"
            . . . x . . . .
            . . . x . . x .
            . . . . . . . .
            . . x . . . . .
            x x x B . . x .
            . . . . . . . .
            . . . . . x . .
            x . . . . . . .
        "#,
        );

        let targets = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
            . . x . . . . .
            . x . . . . x .
            . . . . . . . .
            . . . . . . . .
            . . . . . . . .
        "#,
        );

        let blocked = BitBoard::from_standard(
            r#"
            . . . . . . . .
            . . . . . . . .
            . . . . . x . .
            . . x . x . . .
            . . . B . . . .
            . . x . x . . .
            . x . . . . . .
            . . . . . . . .
        "#,
        );

        let mut filled = start.clone();
        for ray in Ray::DIAGONALS {
            filled |= ray.slides(start, !occupied, targets);
        }

        assert_eq!(filled.to_string(), blocked.to_string());
    }
}
