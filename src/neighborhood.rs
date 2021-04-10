use crate::coord::Coord;

pub struct Neighborhood(Coord);

impl Neighborhood {
    pub fn new<C: Into<Coord>>(coord: C) -> Self {
        Self(coord.into())
    }

    /// Returns the orthogonal and diagonal (Moore) neighborhood of `coord`.
    pub fn iter(&self) -> impl Iterator<Item = Coord> + '_ {
        [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ]
        .iter()
        .map(move |&offset| self.0 + offset.into())
    }

    /// Returns the orthogonal (Von Neumann) neighborhood of `coord`.
    pub fn iter_ortho(&self) -> impl Iterator<Item = Coord> + '_ {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }

    /// Returns the diagonal neighborhood of `coord` (for completeness).
    pub fn iter_diag(&self) -> impl Iterator<Item = Coord> + '_ {
        [(1, 1), (1, -1), (-1, -1), (-1, 1)]
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }
}
