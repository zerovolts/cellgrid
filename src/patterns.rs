//! Iterators over common [`Coord`](crate::coord::Coord) shapes and patterns.
//!
//! These patterns have no dependencies on actual cell data. Their intended use
//! is for ultimately passing into
//! [`Grid::selection_iter`](crate::grid::Grid::selection_iter) or
//! [`Grid::selection_iter_mut`](crate::grid::Grid::selection_iter_mut) to obtain
//! actual cell values.

use std::{collections::HashSet, iter::FromIterator};

use crate::coord::Coord;

/// Returns the orthogonal and diagonal (Moore) neighborhood of `coord`.
pub fn neighborhood<'a, C: Into<Coord>>(coord: C) -> impl Iterator<Item = Coord> + 'a {
    let coord = coord.into();
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
    .map(move |&offset| coord + offset.into())
}

/// Returns the orthogonal (Von Neumann) neighborhood of `coord`.
pub fn ortho_neighborhood<'a, C: Into<Coord>>(coord: C) -> impl Iterator<Item = Coord> + 'a {
    let coord = coord.into();
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(move |&offset| coord + offset.into())
}

/// Returns the diagonal neighborhood of `coord` (for completeness).
pub fn diag_neighborhood<'a, C: Into<Coord>>(coord: C) -> impl Iterator<Item = Coord> + 'a {
    let coord = coord.into();
    [(1, 1), (1, -1), (-1, -1), (-1, 1)]
        .iter()
        .map(move |&offset| coord + offset.into())
}

/// Traces Bresenham's line algorithm between `from` and `to`.
pub fn line(from: Coord, to: Coord) -> impl Iterator<Item = Coord> {
    let delta = to - from;
    let x_step = Coord::new(delta.x.signum(), 0);
    let y_step = Coord::new(0, delta.y.signum());
    let x_is_major = delta.x.abs() > delta.y.abs();

    let (major_step, minor_step) = if x_is_major {
        (x_step, y_step)
    } else {
        (y_step, x_step)
    };

    let (major_fault, minor_fault) = if x_is_major {
        (delta.x.abs(), delta.y.abs())
    } else {
        (delta.y.abs(), delta.x.abs())
    };

    LineIter {
        end_coord: to,
        next_coord: from,
        major_step,
        minor_step,
        fault: major_fault as f32 / 2.0,
        major_fault,
        minor_fault,
        is_finished: false,
    }
}

struct LineIter {
    end_coord: Coord,
    next_coord: Coord,
    // Added to the coordinate every iteration.
    major_step: Coord,
    // Added to the coordinate when `fault` falls below zero.
    minor_step: Coord,
    fault: f32,
    // Amount to add to `fault` when it falls below zero.
    major_fault: i32,
    // Amount to subtract from `fault` every iteration.
    minor_fault: i32,
    is_finished: bool,
}

impl Iterator for LineIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        if self.is_finished {
            return None;
        }
        if self.next_coord == self.end_coord {
            self.is_finished = true;
            return Some(self.end_coord);
        }

        // We return the coordinate computed on the previous iteration.
        let return_coord = self.next_coord;

        self.next_coord += self.major_step;

        self.fault -= self.minor_fault as f32;
        // The choice of < over <= here seems arbitrary. The step patterns they
        // produce are mirror images of each other, for example:
        //  < 0.0 -- 3-4-4-5-4-3
        // <= 0.0 -- 3-4-5-4-4-3
        if self.fault < 0.0 {
            self.fault += self.major_fault as f32;
            self.next_coord += self.minor_step;
        }

        Some(return_coord)
    }
}

/// Represents various layers of a selection of coords (cluster).
///
/// The internal and external borders straddle the "actual edge" of a coord cluster.
#[derive(Debug)]
pub struct ClusterLayers {
    /// The set of cluster coords that does not touch the exterior of the cluster
    /// in any way. For a filled shape, this should represent the majority of the
    /// coords.
    ///
    /// Defined as a cluster cell adjacent only to other cluster cells.
    pub interior: Vec<Coord>,
    /// The border layer lining the inside of a cluster of coords, separating it
    /// from the exterior. This is the layer between the `interior` and `external_border`.
    ///
    /// Defined as a cluster cell adjacent to at least one non-cluster cell.
    pub internal_border: Vec<Coord>,
    /// The border layer surrounding the cluster of coords on the outside. These
    /// coords are not actually part of the cluster itself, but are adjacent to
    /// the `internal_border`.
    ///
    /// Defined as a non-cluster cell adjacent to at least one cluster cell.
    pub external_border: Vec<Coord>,
}

/// Sorts a selection of coords into buckets representing a few fixed layers.
pub fn cluster_layers(cluster: Vec<Coord>) -> ClusterLayers {
    let cluster_set: HashSet<Coord> = HashSet::from_iter(cluster.iter().map(|&x| x));

    let mut internal_boundary = HashSet::new();
    let mut external_boundary = HashSet::new();
    let mut interior = HashSet::new();

    for coord in cluster {
        let non_cluster_coords = neighborhood(coord)
            .filter(|neighbor| !cluster_set.contains(neighbor))
            .collect::<Vec<_>>();
        if non_cluster_coords.len() == 0 {
            // Only adjacent to cluster coords.
            interior.insert(coord);
        } else {
            // Adjacent to at least one non-cluster coord.
            internal_boundary.insert(coord);
            // Add all the adjacent non-cluster coords to the
            // `external_boundary`.
            for neighbor in non_cluster_coords {
                external_boundary.insert(neighbor);
            }
        }
    }

    ClusterLayers {
        internal_border: Vec::from_iter(internal_boundary),
        external_border: Vec::from_iter(external_boundary),
        interior: Vec::from_iter(interior),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_coord_cluster_layers() {
        let layers = cluster_layers(vec![Coord::new(0, 0)]);
        assert!(layers.interior.len() == 0);
        assert!(layers.internal_border.len() == 1);
        assert!(layers.external_border.len() == 8);
    }

    #[test]
    fn square_cluster_layers() {
        // 3x3 square
        let cluster = [
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]
        .iter()
        .map(|&x| x.into())
        .collect::<Vec<_>>();

        let layers = cluster_layers(cluster);
        assert!(layers.interior.len() == 1);
        assert!(layers.internal_border.len() == 8);
        assert!(layers.external_border.len() == 16);
    }
}
