use core::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub const UP: Coord = Coord(-1, 0);
pub const RIGHT: Coord = Coord(0, 1);
pub const DOWN: Coord = Coord(1, 0);
pub const LEFT: Coord = Coord(0, -1);

/// Cardinal Directions in clockwise order, starting at North (N, E, S, W)
pub const ORTHOGONAL_DIRECTIONS: [Coord; 4] = [UP, RIGHT, DOWN, LEFT];

/// Cardinal & Ordinal Directions in clockwise order, starting at North (N, NE, E, SE, S, SW, W, NW)
pub const DIAGONAL_DIRECTIONS: [Coord; 8] = [
    Coord(-1, 0),
    Coord(-1, 1),
    Coord(0, 1),
    Coord(1, 1),
    Coord(1, 0),
    Coord(1, -1),
    Coord(0, -1),
    Coord(-1, -1),
];

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord(pub isize, pub isize);

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<isize> for Coord {
    fn mul_assign(&mut self, rhs: isize) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}
