use std::{
    fmt,
    ops::{Add, Sub},
};

/// The coordinate key to a specific [`Grid`](crate::grid::Grid) cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord::new(x, y)
    }
}

impl From<Coord> for (i32, i32) {
    fn from(Coord { x, y }: Coord) -> Self {
        (x, y)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
