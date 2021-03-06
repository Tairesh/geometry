use std::cmp::Ordering::{Equal, Greater, Less};

use super::{Point, Vec2};

pub const DIR8: [Direction; 8] = [
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
];

pub const DIR9: [Direction; 9] = [
    Direction::Here,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    Here,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    #[must_use]
    pub fn from_delta(dx: i32, dy: i32) -> Self {
        match (dx.cmp(&0), dy.cmp(&0)) {
            (Less, Less) => Direction::NorthWest,
            (Less, Equal) => Direction::West,
            (Less, Greater) => Direction::SouthWest,
            (Equal, Less) => Direction::North,
            (Equal, Equal) => Direction::Here,
            (Equal, Greater) => Direction::South,
            (Greater, Less) => Direction::NorthEast,
            (Greater, Equal) => Direction::East,
            (Greater, Greater) => Direction::SouthEast,
        }
    }

    #[must_use]
    pub fn dx(self) -> i32 {
        match self {
            Direction::NorthWest | Direction::West | Direction::SouthWest => -1,
            Direction::NorthEast | Direction::East | Direction::SouthEast => 1,
            Direction::North | Direction::South | Direction::Here => 0,
        }
    }

    #[must_use]
    pub fn dy(self) -> i32 {
        match self {
            Direction::NorthEast | Direction::North | Direction::NorthWest => -1,
            Direction::SouthEast | Direction::South | Direction::SouthWest => 1,
            Direction::East | Direction::West | Direction::Here => 0,
        }
    }

    #[must_use]
    pub fn is_here(self) -> bool {
        self == Direction::Here
    }
}

impl From<(i32, i32)> for Direction {
    fn from((dx, dy): (i32, i32)) -> Self {
        Self::from_delta(dx, dy)
    }
}

impl From<Point> for Direction {
    fn from(point: Point) -> Self {
        Self::from_delta(point.x, point.y)
    }
}

impl From<Direction> for Vec2 {
    #[allow(clippy::cast_precision_loss)]
    fn from(dir: Direction) -> Self {
        Vec2::new(dir.dx() as f32, dir.dy() as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Point};

    #[test]
    fn from_delta() {
        let dir = Direction::from_delta(10, 20);
        assert!(matches!(dir, Direction::SouthEast));
    }

    #[test]
    fn from_tuple() {
        let dir = Direction::from((10, 20));
        assert!(matches!(dir, Direction::SouthEast));
    }

    #[test]
    fn from_point() {
        let dir = Direction::from(Point::new(10, 20));
        assert!(matches!(dir, Direction::SouthEast));
    }

    #[test]
    fn from_point_diff() {
        let pt = Point::new(1, 2);
        let dir = pt.dir_to(Point::new(3, 4));
        assert!(matches!(dir, Direction::SouthEast));
    }
}
