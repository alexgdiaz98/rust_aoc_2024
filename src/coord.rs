use core::fmt;
use std::{
    fmt::write,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

pub const UP: Coord = Coord(-1, 0);
pub const RIGHT: Coord = Coord(0, 1);
pub const DOWN: Coord = Coord(1, 0);
pub const LEFT: Coord = Coord(0, -1);

/// Cardinal Directions in clockwise order, starting at North (N, E, S, W)
pub const ORTHOGONAL_DIRECTIONS: [Coord; 4] = [UP, RIGHT, DOWN, LEFT];
pub trait ToCoord {
    fn coord(&self) -> Coord;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum OrthogonalDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl fmt::Debug for OrthogonalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.coord();
        write!(f, "[{},{}]", c.0, c.1)
    }
}

impl fmt::Display for OrthogonalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UP => write!(f, "↑"),
            Self::RIGHT => write!(f, "→"),
            Self::DOWN => write!(f, "↓"),
            Self::LEFT => write!(f, "←"),
        }
    }
}

impl OrthogonalDirection {
    #[inline]
    pub const fn coord_of(dir: &Self) -> Coord {
        match dir {
            OrthogonalDirection::UP => Coord(-1, 0),
            OrthogonalDirection::RIGHT => Coord(0, 1),
            OrthogonalDirection::DOWN => Coord(1, 0),
            OrthogonalDirection::LEFT => Coord(0, -1),
        }
    }

    pub fn coord(&self) -> Coord {
        OrthogonalDirection::coord_of(self)
    }

    pub fn cw(&self) -> Self {
        match self {
            OrthogonalDirection::UP => OrthogonalDirection::RIGHT,
            OrthogonalDirection::RIGHT => OrthogonalDirection::DOWN,
            OrthogonalDirection::DOWN => OrthogonalDirection::LEFT,
            OrthogonalDirection::LEFT => OrthogonalDirection::UP,
        }
    }

    pub fn ccw(&self) -> Self {
        match self {
            OrthogonalDirection::UP => OrthogonalDirection::LEFT,
            OrthogonalDirection::RIGHT => OrthogonalDirection::UP,
            OrthogonalDirection::DOWN => OrthogonalDirection::RIGHT,
            OrthogonalDirection::LEFT => OrthogonalDirection::DOWN,
        }
    }
}

impl ToCoord for OrthogonalDirection {
    fn coord(&self) -> Coord {
        OrthogonalDirection::coord_of(self)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum HorizontalDirection {
    RIGHT,
    LEFT,
}

impl fmt::Debug for HorizontalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.coord();
        write!(f, "[{},{}]", c.0, c.1)
    }
}

impl fmt::Display for HorizontalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.coord();
        write!(f, "[{},{}]", c.0, c.1)
    }
}

impl HorizontalDirection {
    #[inline]
    pub const fn coord_of(dir: &Self) -> Coord {
        match dir {
            HorizontalDirection::RIGHT => Coord(0, 1),
            HorizontalDirection::LEFT => Coord(0, -1),
        }
    }
}

impl From<HorizontalDirection> for OrthogonalDirection {
    fn from(val: HorizontalDirection) -> Self {
        match val {
            HorizontalDirection::LEFT => OrthogonalDirection::LEFT,
            HorizontalDirection::RIGHT => OrthogonalDirection::RIGHT,
        }
    }
}

impl From<OrthogonalDirection> for HorizontalDirection {
    fn from(val: OrthogonalDirection) -> Self {
        match val {
            OrthogonalDirection::LEFT => HorizontalDirection::LEFT,
            OrthogonalDirection::RIGHT => HorizontalDirection::RIGHT,
            _ => panic!(
                "Orthogonal direction {} cannot be cast into HorizontalDirection",
                val
            ),
        }
    }
}

impl ToCoord for HorizontalDirection {
    fn coord(&self) -> Coord {
        Self::coord_of(self)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum VerticalDirection {
    UP,
    DOWN,
}

impl fmt::Debug for VerticalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.coord();
        write!(f, "[{},{}]", c.0, c.1)
    }
}

impl fmt::Display for VerticalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.coord();
        write!(f, "[{},{}]", c.0, c.1)
    }
}

impl VerticalDirection {
    #[inline]
    pub const fn coord_of(dir: &Self) -> Coord {
        match dir {
            VerticalDirection::UP => Coord(-1, 0),
            VerticalDirection::DOWN => Coord(1, 0),
        }
    }
}

impl From<VerticalDirection> for OrthogonalDirection {
    fn from(val: VerticalDirection) -> Self {
        match val {
            VerticalDirection::DOWN => OrthogonalDirection::DOWN,
            VerticalDirection::UP => OrthogonalDirection::UP,
        }
    }
}

impl From<OrthogonalDirection> for VerticalDirection {
    fn from(val: OrthogonalDirection) -> Self {
        match val {
            OrthogonalDirection::DOWN => VerticalDirection::DOWN,
            OrthogonalDirection::UP => VerticalDirection::UP,
            _ => panic!(
                "Orthogonal direction {} cannot be cast into VerticalDirection",
                val
            ),
        }
    }
}

impl ToCoord for VerticalDirection {
    fn coord(&self) -> Coord {
        Self::coord_of(self)
    }
}

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

impl fmt::Display for Coord {
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
