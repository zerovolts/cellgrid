//! Iterators over common [`Coord`] shapes and patterns.
//!
//! The patterns have no dependencies on actual cell data. They are relative to
//! the origin and should be anchored to a concrete Coord before use.

use crate::coord::Coord;

/// Returns the orthogonal and diagonal (Moore) neighborhood of `self`.
pub fn neighbor_coords<'a>() -> impl Iterator<Item = Coord> + 'a {
    [
        Coord(0, 1),
        Coord(1, 1),
        Coord(1, 0),
        Coord(1, -1),
        Coord(0, -1),
        Coord(-1, -1),
        Coord(-1, 0),
        Coord(-1, 1),
    ]
    .iter()
    .map(|&x| x)
}

/// Returns the orthogonal (Von Neumann) neighborhood of `self`.
pub fn ortho_neighbor_coords<'a>() -> impl Iterator<Item = Coord> + 'a {
    [Coord(0, 1), Coord(1, 0), Coord(0, -1), Coord(-1, 0)]
        .iter()
        .map(|&x| x)
}

/// Returns the diagonal neighborhood of `self` (for completeness).
pub fn diag_neighbor_coords<'a>() -> impl Iterator<Item = Coord> + 'a {
    [Coord(1, 1), Coord(1, -1), Coord(-1, -1), Coord(-1, 1)]
        .iter()
        .map(|&x| x)
}
