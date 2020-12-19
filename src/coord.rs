use std::ops::{Add, Sub};

/// The coordinate key to a specific [`Grid`](crate::grid::Grid) cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub i32, pub i32);

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord(x, y)
    }
}

impl From<Coord> for (i32, i32) {
    fn from(Coord(x, y): Coord) -> Self {
        (x, y)
    }
}
