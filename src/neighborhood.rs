use crate::coord::Coord;

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

const ORTHO_NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const DIAG_NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

pub struct Neighborhood(Coord);

impl Neighborhood {
    pub fn new<C: Into<Coord>>(coord: C) -> Self {
        Self(coord.into())
    }

    /// Returns the orthogonal and diagonal (Moore) neighborhood of `coord`.
    pub fn iter(&self) -> impl Iterator<Item = Coord> + '_ {
        NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }

    /// Returns the orthogonal and diagonal (Moore) neighborhood of `coord`.
    pub fn into_iter(self) -> impl Iterator<Item = Coord> {
        NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }

    /// Returns the orthogonal (Von Neumann) neighborhood of `coord`.
    pub fn iter_ortho(&self) -> impl Iterator<Item = Coord> + '_ {
        ORTHO_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }

    /// Returns the orthogonal (Von Neumann) neighborhood of `coord`.
    pub fn into_iter_ortho(self) -> impl Iterator<Item = Coord> {
        ORTHO_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }

    /// Returns the diagonal neighborhood of `coord` (for completeness).
    pub fn iter_diag(&self) -> impl Iterator<Item = Coord> + '_ {
        DIAG_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }

    /// Returns the diagonal neighborhood of `coord` (for completeness).
    pub fn into_iter_diag(self) -> impl Iterator<Item = Coord> {
        DIAG_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset.into())
    }
}
