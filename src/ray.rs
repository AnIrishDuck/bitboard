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
    pub fn shift(&self) -> u32 {
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
    pub fn step(self, board: BitBoard) -> BitBoard {
        let wrap = self.wraps();
        let stay_if_wrap = board & wrap;
        let step_if_no_wrap = (board & !wrap).rotate_left(self.shift());
        step_if_no_wrap | stay_if_wrap
    }

    #[inline]
    fn step_mask(self, board: &mut BitBoard, mask: BitBoard) -> BitBoard {
        *board = board.rotate_left(self.shift()) & mask;
        *board
    }

    pub fn dumb7(self, mut sliders: BitBoard, mut empty: BitBoard) -> BitBoard {
        let mut flood = sliders.clone();
        let nowrap = !self.wraps();
        empty &= nowrap;
        flood |= self.step_mask(&mut sliders, empty);
        flood |= self.step_mask(&mut sliders, empty);
        flood |= self.step_mask(&mut sliders, empty);
        flood |= self.step_mask(&mut sliders, empty);
        flood |= self.step_mask(&mut sliders, empty);
        flood |= self.step_mask(&mut sliders, empty);
        flood | self.step_mask(&mut sliders, nowrap)
    }

    pub fn moves_with_capture(
        self,
        sliders: BitBoard,
        empty: BitBoard,
        others: BitBoard,
    ) -> BitBoard {
        let mut quiet = self.dumb7(sliders, empty);
        quiet | self.step_mask(&mut quiet, others)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{Rank, Square};
    use pretty_assertions::assert_eq;

    fn assert_step(from: Square, ray: Ray, to: Square) {
        let start = BitBoard::from_square(from);
        let actual = ray.step(start);
        let expected = BitBoard::from_square(to);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    fn assert_no_step(from: Square, ray: Ray) {
        let start = BitBoard::from_square(from);
        let actual = ray.step(start);
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
}
