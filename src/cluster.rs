use std::{
    collections::{hash_set::Iter, HashSet, VecDeque},
    iter::FromIterator,
};

use crate::{coord::Coord, neighborhood::Neighborhood};

/// Represents various "layers" of a selection of coords (cluster).
///
/// The internal and external borders straddle the "actual edge" of a coord cluster.
pub struct Cluster(pub HashSet<Coord>);

impl Cluster {
    pub fn new(iter: impl Iterator<Item = Coord>) -> Self {
        Self(HashSet::from_iter(iter))
    }

    /// The set of cluster coords that does not touch the exterior of the cluster
    /// in any way. For a filled shape, this should represent the majority of the
    /// coords.
    ///
    /// Defined as a cluster cell adjacent only to other cluster cells.
    pub fn iter_interior(&self) -> impl Iterator<Item = Coord> + '_ {
        self.0.iter().filter_map(move |&coord| {
            // If this coord has external neighbors, then it's on the internal border.
            if self.external_neighbors(coord).count() != 0 {
                return None;
            }

            Some(coord)
        })
    }

    /// The border layer lining the inside of a cluster of coords, separating it
    /// from the exterior. This is the layer between the `interior` and `external_border`.
    ///
    /// Defined as a cluster cell adjacent to at least one non-cluster cell.
    pub fn iter_internal_border(&self) -> impl Iterator<Item = Coord> + '_ {
        self.0.iter().filter_map(move |&coord| {
            // If this coord has no external neighbors, then it's on the interior.
            if self.external_neighbors(coord).count() == 0 {
                return None;
            }

            Some(coord)
        })
    }

    // /// The border layer surrounding the cluster of coords on the outside. These
    // /// coords are not actually part of the cluster itself, but are adjacent to
    // /// the `internal_border`.
    // ///
    // /// Defined as a non-cluster cell adjacent to at least one cluster cell.
    pub fn iter_external_border(&self) -> ExternalBorderIter {
        ExternalBorderIter {
            cluster: self,
            coords: self.0.iter(),
            external_border_coords: HashSet::new(),
            coords_to_return: VecDeque::new(),
        }
    }

    fn external_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        Neighborhood::new(coord)
            .into_iter()
            .filter(move |neighbor| !self.0.contains(neighbor))
    }
}

pub struct ExternalBorderIter<'a> {
    /// Used for finding external neighbors.
    cluster: &'a Cluster,
    /// All cluster coords to iterate through.
    coords: Iter<'a, Coord>,
    /// Coords that have been found to be on the external border.
    external_border_coords: HashSet<Coord>,
    /// External border coords that haven't been returned yet.
    coords_to_return: VecDeque<Coord>,
}

impl<'a> Iterator for ExternalBorderIter<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are no external border coords to return, loop through the
        // cluster to find external neighbors until new ones are found.
        if self.coords_to_return.len() == 0 {
            while let Some(coord) = self.coords.next() {
                let external_neighbors = self.cluster.external_neighbors(*coord);
                for neighbor in external_neighbors {
                    if !self.external_border_coords.contains(&neighbor) {
                        self.external_border_coords.insert(neighbor);
                        self.coords_to_return.push_back(neighbor);
                    }
                }
            }
        }

        if self.coords_to_return.len() > 0 {
            return self.coords_to_return.pop_front();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_coord_cluster_layers() {
        let cluster = Cluster::new(vec![Coord::new(0, 0)].iter().map(|&c| c));
        assert!(cluster.iter_interior().count() == 0);
        assert!(cluster.iter_internal_border().count() == 1);
        assert!(cluster.iter_external_border().count() == 8);
    }

    #[test]
    fn square_cluster_layers() {
        // 3x3 square
        let cluster = Cluster::new(
            [
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
            .map(|&x| x.into()),
        );

        assert!(cluster.iter_interior().count() == 1);
        assert!(cluster.iter_internal_border().count() == 8);
        assert!(cluster.iter_external_border().count() == 16);
    }
}
