use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    /// Offsets the given `relative_coords` by `self`.
    pub fn anchor_coords<'a>(
        &'a self,
        relative_coords: &'a [Coord],
    ) -> impl Iterator<Item = Coord> + 'a {
        relative_coords.iter().map(move |&coord| *self + coord)
    }

    /// Returns the orthogonal and diagonal (Moore) neighborhood of `self`.
    pub fn neighbor_coords<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        self.anchor_coords(&[
            Coord(0, 1),
            Coord(1, 1),
            Coord(1, 0),
            Coord(1, -1),
            Coord(0, -1),
            Coord(-1, -1),
            Coord(-1, 0),
            Coord(-1, 1),
        ])
    }

    /// Returns the orthogonal (Von Neumann) neighborhood of `self`.
    pub fn ortho_neighbor_coords<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        self.anchor_coords(&[Coord(0, 1), Coord(1, 0), Coord(0, -1), Coord(-1, 0)])
    }

    /// Returns the diagonal neighborhood of `self` (for completeness).
    pub fn diag_neighbor_coords<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        self.anchor_coords(&[Coord(1, 1), Coord(1, -1), Coord(-1, -1), Coord(-1, 1)])
    }
}

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
