use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord(pub isize, pub isize);

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
