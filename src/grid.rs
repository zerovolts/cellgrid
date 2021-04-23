use std::mem;

use crate::coord::Coord;

/// The return type of all Grid iterators; a tuple of the cell coordinate and a
/// reference to the cell data.
pub type IterCell<'a, T> = (Coord, &'a T);

/// The return type of all mutable Grid iterators; a tuple of the cell coordinate
/// and a mutable reference to the cell data.
pub type IterCellMut<'a, T> = (Coord, &'a mut T);

#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    /// The coordinate has no cell associated with it, as it's out of the grid
    /// bounds.
    OutOfBounds(Coord),
    /// The coordinate has previously been mutably borrowed from the iterator,
    /// and doing so again would break safety guarantees.
    AlreadyVisited(Coord),
}

pub trait Grid<T> {
    fn get<C: Into<Coord>>(&self, coord: C) -> Option<&T>;

    fn get_mut<C: Into<Coord>>(&mut self, coord: C) -> Option<&mut T>;

    fn copy<C1, C2>(&mut self, src: C1, dest: C2) -> bool
    where
        T: Copy,
        C1: Into<Coord>,
        C2: Into<Coord>;

    /// Swaps the contents of two cells.
    fn swap<C1, C2>(&mut self, coord1: C1, coord2: C2) -> bool
    where
        C1: Into<Coord>,
        C2: Into<Coord>;

    /// Moves the contents of `src` into `dest`, returning the previous contents
    /// of `dest`.
    fn mov(&mut self, src: Coord, dest: Coord) -> Option<T>
    where
        T: Default;

    //
    // DEFAULT IMPLEMENTATIONS
    //

    fn set<C: Into<Coord>>(&mut self, coord: C, value: T) -> bool {
        if let Some(cell) = self.get_mut(coord) {
            *cell = value;
            true
        } else {
            false
        }
    }

    fn replace<C: Into<Coord>>(&mut self, coord: C, value: T) -> Option<T> {
        self.get_mut(coord)
            .and_then(|cell| Some(mem::replace(cell, value)))
    }

    fn take<C: Into<Coord>>(&mut self, coord: C) -> Option<T>
    where
        T: Default,
    {
        self.get_mut(coord).and_then(|cell| Some(mem::take(cell)))
    }
}
