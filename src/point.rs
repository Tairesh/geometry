use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use vek::num_traits::Zero;

use super::{Direction, Vec2};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[cfg(feature = "rand")]
    pub fn random<R: rand::Rng + ?Sized>(
        rng: &mut R,
        horizontal: std::ops::Range<i32>,
        vertical: std::ops::Range<i32>,
    ) -> Self {
        Self::new(rng.gen_range(horizontal), rng.gen_range(vertical))
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    /// Helper for map index conversion
    pub fn to_index(self, width: i32) -> Option<usize> {
        if self.x < 0 || self.y < 0 || self.x >= width {
            None
        } else {
            Some(((self.y * width) + self.x) as usize)
        }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    /// Helper for map index conversion
    pub fn from_index(index: usize, width: i32) -> Point {
        Point::new(index as i32 % width, index as i32 / width)
    }

    #[must_use]
    /// Direction to other point
    pub fn dir_to(self, other: Point) -> Direction {
        Direction::from(other - self)
    }

    #[must_use]
    /// Square distance to other point
    pub fn square_distance(self, other: Self) -> u32 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx * dx + dy * dy
    }

    #[must_use]
    /// Distance (pythagorean) to other point
    pub fn distance(self, other: Self) -> f64 {
        f64::sqrt(f64::from(self.square_distance(other)))
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl From<Point> for (i32, i32) {
    fn from(pos: Point) -> Self {
        (pos.x, pos.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Point> for Vec2 {
    #[allow(clippy::cast_precision_loss)]
    fn from(point: Point) -> Self {
        Self::new(point.x as f32, point.y as f32)
    }
}

impl From<Vec2> for Point {
    #[allow(clippy::cast_possible_truncation)]
    fn from(vec: Vec2) -> Self {
        Self::new(vec.x.round() as i32, vec.y.round() as i32)
    }
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        Self::new(dir.dx(), dir.dy())
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        Self::new(self.x + rhs.dx(), self.y + rhs.dy())
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<Vec2> for Point {
    type Output = Point;

    fn add(self, rhs: Vec2) -> Self::Output {
        (Vec2::from(self) + rhs).into()
    }
}

impl Sub<Direction> for Point {
    type Output = Point;

    fn sub(self, rhs: Direction) -> Self::Output {
        Self::new(self.x - rhs.dx(), self.y - rhs.dy())
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Point;

    fn sub(self, (dx, dy): (i32, i32)) -> Self::Output {
        Self::new(self.x - dx, self.y - dy)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Vec2> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec2) -> Self::Output {
        (Vec2::from(self) - rhs).into()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, dir: Direction) {
        self.x += dir.dx();
        self.y += dir.dy();
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<(i32, i32)> for Point {
    type Output = Point;

    fn mul(self, (mx, my): (i32, i32)) -> Self::Output {
        Self::new(self.x * mx, self.y * my)
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        (Vec2::from(self) * rhs).into()
    }
}

impl Mul<(f32, f32)> for Point {
    type Output = Point;

    fn mul(self, (mx, my): (f32, f32)) -> Self::Output {
        (Vec2::from(self) * Vec2::new(mx, my)).into()
    }
}

impl Mul<Vec2> for Point {
    type Output = Point;

    fn mul(self, rhs: Vec2) -> Self::Output {
        (rhs * Vec2::from(self)).into()
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl MulAssign<(i32, i32)> for Point {
    fn mul_assign(&mut self, rhs: (i32, i32)) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl MulAssign<(f32, f32)> for Point {
    fn mul_assign(&mut self, rhs: (f32, f32)) {
        *self = *self * rhs;
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, rhs: Point) {
        *self = *self * rhs;
    }
}

impl MulAssign<Vec2> for Point {
    fn mul_assign(&mut self, rhs: Vec2) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Point, Vec2};

    #[test]
    fn index_converting() {
        let pt = Point::new(1, 2);
        assert_eq!(21, pt.to_index(10).unwrap());
        let pt2 = Point::from_index(21, 10);
        assert_eq!(pt, pt2);
        let pt = Point::new(-1, 2);
        assert!(pt.to_index(10).is_none());
        let pt = Point::new(1, -2);
        assert!(pt.to_index(10).is_none());
        let pt = Point::new(10, 2);
        assert!(pt.to_index(10).is_none());
    }

    #[test]
    fn add_direction_to_point() {
        let mut pt = Point::new(1, 2);
        pt += Direction::NorthWest;
        assert_eq!(0, pt.x);
        assert_eq!(1, pt.y);
    }

    #[test]
    fn mul_point_to_float_tuple() {
        let pt = Point::new(1, 2);
        let pt2 = pt * (1.4, 2.2);
        assert_eq!(1, pt2.x);
        assert_eq!(4, pt2.y);
    }

    #[test]
    fn mul_point_to_vec() {
        let pt = Point::new(2, 3);
        let pt2 = pt * Vec2::new(1.5, 2.0);
        assert_eq!(3, pt2.x);
        assert_eq!(6, pt2.y);
    }

    #[test]
    fn test_dist() {
        let pt = Point::new(1, 2);
        let pt2 = Point::new(3, 4);
        assert_eq!(8, pt.square_distance(pt2));
        assert!(f64::abs(pt.distance(pt2) - 2.828_427_124_746_190_3) < f64::EPSILON);
    }

    #[test]
    fn mul_assign() {
        let mut pt = Point::new(1, 2);
        pt *= (1.5, 2.0);
        assert_eq!(2, pt.x);
        assert_eq!(4, pt.y);

        let mut pt = Point::new(1, 2);
        pt *= Vec2::new(1.5, 2.0);
        assert_eq!(2, pt.x);
        assert_eq!(4, pt.y);
    }
}
