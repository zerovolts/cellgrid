use crate::coord::Coord;

const NEIGHBOR_OFFSETS: [Coord; 8] = [
    Coord::NORTH,
    Coord::NORTH_EAST,
    Coord::EAST,
    Coord::SOUTH_EAST,
    Coord::SOUTH,
    Coord::SOUTH_WEST,
    Coord::WEST,
    Coord::NORTH_WEST,
];

const ORTHO_NEIGHBOR_OFFSETS: [Coord; 4] = [Coord::NORTH, Coord::EAST, Coord::SOUTH, Coord::WEST];

const DIAG_NEIGHBOR_OFFSETS: [Coord; 4] = [
    Coord::NORTH_EAST,
    Coord::SOUTH_EAST,
    Coord::SOUTH_WEST,
    Coord::NORTH_WEST,
];

pub struct Neighborhood(Coord);

impl Neighborhood {
    pub fn new<C: Into<Coord>>(coord: C) -> Self {
        Self(coord.into())
    }

    /// Returns the orthogonal and diagonal (Moore) neighborhood of `coord`.
    pub fn iter(&self) -> impl Iterator<Item = Coord> + '_ {
        NEIGHBOR_OFFSETS.iter().map(move |&offset| self.0 + offset)
    }

    /// Returns the orthogonal and diagonal (Moore) neighborhood of `coord`.
    pub fn into_iter(self) -> impl Iterator<Item = Coord> {
        NEIGHBOR_OFFSETS.iter().map(move |&offset| self.0 + offset)
    }

    /// Returns the orthogonal (Von Neumann) neighborhood of `coord`.
    pub fn iter_ortho(&self) -> impl Iterator<Item = Coord> + '_ {
        ORTHO_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset)
    }

    /// Returns the orthogonal (Von Neumann) neighborhood of `coord`.
    pub fn into_iter_ortho(self) -> impl Iterator<Item = Coord> {
        ORTHO_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset)
    }

    /// Returns the diagonal neighborhood of `coord` (for completeness).
    pub fn iter_diag(&self) -> impl Iterator<Item = Coord> + '_ {
        DIAG_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset)
    }

    /// Returns the diagonal neighborhood of `coord` (for completeness).
    pub fn into_iter_diag(self) -> impl Iterator<Item = Coord> {
        DIAG_NEIGHBOR_OFFSETS
            .iter()
            .map(move |&offset| self.0 + offset)
    }
}
