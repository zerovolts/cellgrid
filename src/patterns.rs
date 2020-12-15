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

// Bresenham's line algorithm
pub fn line(from: Coord, to: Coord) -> impl Iterator<Item = Coord> {
    let delta = to - from;
    let x_step = Coord(delta.0.signum(), 0);
    let y_step = Coord(0, delta.1.signum());
    let x_is_major = delta.0.abs() > delta.1.abs();

    let (major_step, minor_step) = if x_is_major {
        (x_step, y_step)
    } else {
        (y_step, x_step)
    };

    let (major_fault, minor_fault) = if x_is_major {
        (delta.0.abs(), delta.1.abs())
    } else {
        (delta.1.abs(), delta.0.abs())
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

        // We return the coordinate computed on the previous iteration
        let return_coord = self.next_coord;

        // TODO: AddAssign
        self.next_coord = self.next_coord + self.major_step;

        self.fault -= self.minor_fault as f32;
        // < vs <= here?
        if self.fault < 0.0 {
            self.fault += self.major_fault as f32;
            // TODO: AddAssign
            self.next_coord = self.next_coord + self.minor_step;
        }

        Some(return_coord)
    }
}
