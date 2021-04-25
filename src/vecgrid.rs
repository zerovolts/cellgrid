use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use crate::{
    coord::Coord,
    grid::{Grid, GridError, IterCell, IterCellMut},
    patterns::{Neighborhood, Rect},
};

/// The core type of this library. A 2D grid of cell type `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecGrid<T> {
    /// Row-major, linear storage of cell data.
    pub cells: Vec<T>,
    pub bounds: Rect,
}

impl<T> Grid<T> for VecGrid<T> {
    fn get<C: Into<Coord>>(&self, coord: C) -> Option<&T> {
        self.cells.get(self.coord_to_index(coord)?)
    }

    fn get_mut<C: Into<Coord>>(&mut self, coord: C) -> Option<&mut T> {
        let index = self.coord_to_index(coord)?;
        self.cells.get_mut(index)
    }

    fn copy<C1, C2>(&mut self, src: C1, dest: C2) -> bool
    where
        T: Copy,
        C1: Into<Coord>,
        C2: Into<Coord>,
    {
        if let Some(src_index) = self.coord_to_index(src) {
            if let Some(dest_index) = self.coord_to_index(dest) {
                self.cells
                    .copy_within(src_index..(src_index + 1), dest_index);
                return true;
            }
        }
        false
    }

    /// Swaps the contents of two cells.
    fn swap<C1, C2>(&mut self, coord1: C1, coord2: C2) -> bool
    where
        C1: Into<Coord>,
        C2: Into<Coord>,
    {
        if let Some(index1) = self.coord_to_index(coord1) {
            if let Some(index2) = self.coord_to_index(coord2) {
                self.cells.swap(index1, index2);
                return true;
            }
        }
        false
    }

    /// Moves the contents of `src` into `dest`, returning the previous contents
    /// of `dest`.
    fn mov(&mut self, src: Coord, dest: Coord) -> Option<T>
    where
        T: Default,
    {
        // Make sure both coordinates are in bounds before mutating things.
        if !(self.bounds.contains(src) && self.bounds.contains(dest)) {
            return None;
        }
        let src_value = self.take(src).unwrap();
        self.replace(dest, src_value)
    }
}

impl<T> VecGrid<T> {
    pub fn new(bounds: Rect) -> Self
    where
        T: Default + Clone,
    {
        Self {
            cells: vec![T::default(); bounds.area() as usize],
            bounds,
        }
    }

    pub fn with_generator<C>(bounds: Rect, generator: impl Fn(C) -> T) -> Self
    where
        C: From<Coord>,
    {
        let mut cells = Vec::with_capacity(bounds.area() as usize);
        // TODO: Implement an iterator over all grid cells.
        for y in bounds.y_range() {
            for x in bounds.x_range() {
                let coord = Coord::new(x, y);
                cells.push(generator(coord.into()));
            }
        }
        Self { cells, bounds }
    }

    /// Returns an iterator over all cells in the grid.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = IterCell<'a, T>> {
        Box::new(
            self.cells
                .iter()
                .clone()
                .enumerate()
                .map(move |cell| (self.index_to_coord(cell.0), cell.1)),
        )
    }

    /// Returns a mutable iterator over all cells in the grid.
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = IterCellMut<'a, T>> {
        let rect = self.bounds;
        self.cells
            .iter_mut()
            .enumerate()
            .map(move |(index, cell)| (Self::index_to_coord_with_bounds(rect, index), cell))
    }

    /// Returns an iterator over the cells specified by the coords iterator.
    pub fn selection_iter<I>(&self, coords: I) -> SelectionIter<T, I>
    where
        I: Iterator<Item = Coord>,
    {
        SelectionIter { grid: self, coords }
    }

    /// Returns a mutable iterator over the cells specified by the coords
    /// iterator.
    ///
    /// If there is an attempt to visit a given cell more than once (which would
    /// create multiple simultaneous mutable references to the cell), a
    /// [`GridError::AlreadyVisited`](GridError::AlreadyVisited) will be returned
    /// in place of the cell contents.
    pub fn selection_iter_mut<I>(&mut self, coords: I) -> SelectionIterMut<T, I>
    where
        I: Iterator<Item = Coord>,
    {
        SelectionIterMut {
            grid: self,
            coords,
            visited_coords: HashSet::new(),
        }
    }

    /// Returns an iterator beginning from `starting_coord` and continuing
    /// through all recursively adjacent coords that satisfy the `predicate`. In
    /// other words, this iterates through the cells according to a flood fill
    /// algorithm.
    ///
    /// Since there is no `mut` version of this iterator (which would require
    /// simultaneous mutable and shared references to most of the cells), the
    /// resulting iterator can be collected and then passed into
    /// [`Grid::selection_iter_mut`](crate::grid::Grid::selection_iter_mut) to
    /// gain access to mutable cell contents.
    pub fn flood_iter<C: Into<Coord>>(
        &self,
        starting_coord: C,
        predicate: impl Fn(&T) -> bool + 'static,
    ) -> FloodIter<T> {
        let mut coords_to_search = VecDeque::new();
        coords_to_search.push_back(starting_coord.into());

        FloodIter {
            grid: self,
            predicate: Box::new(predicate),
            searched_coords: vec![],
            coords_to_search,
        }
    }

    /// Converts a 2D Grid coordinate into a linear Vec index.
    fn coord_to_index<C: Into<Coord>>(&self, coord: C) -> Option<usize> {
        let coord = coord.into();
        if !self.bounds.contains(coord) {
            return None;
        }
        let offset_coord = coord - self.bounds.offset();
        Some((offset_coord.x + offset_coord.y * self.bounds.width()) as usize)
    }

    /// Converts a linear Vec index into a 2D Grid coordinate.
    fn index_to_coord(&self, index: usize) -> Coord {
        Self::index_to_coord_with_bounds(self.bounds, index)
    }

    /// Use `index_to_coord` if possible. This exists so that `iter_mut` can
    /// avoid borrowing `self`.
    fn index_to_coord_with_bounds(bounds: Rect, index: usize) -> Coord {
        let y = (index as f32 / bounds.width() as f32).floor() as i32;
        let x = index as i32 - (y * bounds.width()) as i32;
        Coord::new(x, y) + bounds.offset()
    }
}

pub struct SelectionIter<'a, T, I> {
    // TODO: Generic Grid
    grid: &'a VecGrid<T>,
    coords: I,
}

impl<'a, T, I> Iterator for SelectionIter<'a, T, I>
where
    I: Iterator<Item = Coord>,
{
    type Item = Result<IterCell<'a, T>, GridError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(coord) = self.coords.next() {
            if let Some(cell) = self.grid.get(coord) {
                return Some(Ok((coord, cell)));
            }
            return Some(Err(GridError::OutOfBounds(coord)));
        }
        None
    }
}

pub struct SelectionIterMut<'a, T, I> {
    // TODO: Generic Grid
    grid: &'a mut VecGrid<T>,
    coords: I,
    visited_coords: HashSet<Coord>,
}

impl<'a, T, I> Iterator for SelectionIterMut<'a, T, I>
where
    I: Iterator<Item = Coord>,
{
    type Item = Result<IterCellMut<'a, T>, GridError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(coord) = self.coords.next() {
            if self.visited_coords.contains(&coord) {
                return Some(Err(GridError::AlreadyVisited(coord)));
            }
            if let Some(cell) = self.grid.get_mut(coord).map(|cell| cell as *mut T) {
                // SAFETY: We guarantee that only one mut reference to a cell
                // will be given out at a time by checking each against a list
                // of visited cells and only returning those that havent already
                // visited.
                // This is likely not actually completely safe, given that I
                // don't know which cases to look out for.
                let opt_cell = unsafe { cell.as_mut() };
                if let Some(cell) = opt_cell {
                    self.visited_coords.insert(coord);
                    return Some(Ok((coord, cell)));
                }
            }
            return Some(Err(GridError::OutOfBounds(coord)));
        }
        None
    }
}

pub struct FloodIter<'a, T> {
    // TODO: Generic Grid
    grid: &'a VecGrid<T>,
    predicate: Box<dyn Fn(&T) -> bool>,
    searched_coords: Vec<Coord>,
    coords_to_search: VecDeque<Coord>,
}

impl<'a, T> Iterator for FloodIter<'a, T> {
    type Item = IterCell<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.coords_to_search.len() > 0 {
            let coord = self.coords_to_search.pop_front().unwrap();
            let is_cell_included = self
                .grid
                .get(coord)
                .and_then(|cell| Some((self.predicate)(cell)))
                .unwrap_or(false);

            self.searched_coords.push(coord);

            if !is_cell_included {
                continue;
            }

            let neighbor_coords = Neighborhood::new(coord)
                .iter_ortho()
                .filter(|&coord| {
                    !(self.searched_coords.contains(&coord)
                        || self.coords_to_search.contains(&coord))
                        && self.grid.bounds.contains(coord)
                })
                .collect::<Vec<Coord>>();

            self.coords_to_search.extend(neighbor_coords);

            return Some((coord, self.grid.get(coord).unwrap()));
        }

        None
    }
}

// TODO: Generic Grid
impl<T> fmt::Display for VecGrid<T>
where
    char: From<T>,
    T: Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.bounds.y_range() {
            for x in self.bounds.x_range() {
                let c: char = match self.get((x, y)) {
                    Some(cell) => char::from(*cell),
                    None => '�',
                };
                if let Err(e) = write!(f, "{} ", c) {
                    return Err(e);
                }
            }
            if let Err(e) = write!(f, "\n") {
                return Err(e);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_and_dimensions() {
        let grid = VecGrid::<()>::new(Rect::new((8, 8)));
        assert_eq!(grid.bounds.width(), 8);
    }

    #[test]
    fn out_of_bounds_coords() {
        let grid = VecGrid::<()>::new(Rect::new((8, 8)));
        // This would pass `coord_to_index` if there was no bounds check.
        assert_eq!(grid.get(Coord::new(-1, 4)), None);
    }

    #[test]
    fn test_index_to_coord() {
        let grid = VecGrid::<()>::new(Rect::new((8, 4)));
        assert_eq!(grid.index_to_coord(12), Coord::new(4, 1));
    }

    #[test]
    fn selection_iter_mut() {
        let mut grid: VecGrid<bool> = VecGrid::new(Rect::new((4, 4)));
        // Set all neighbors of (2, 2) to `true`.
        for res_cell in grid.selection_iter_mut(Neighborhood::new((2, 2)).iter()) {
            *res_cell.unwrap().1 = true;
        }
        assert_eq!(grid.get(Coord::new(2, 2)), Some(&false)); // center
        assert_eq!(grid.get(Coord::new(3, 2)), Some(&true)); // right
        assert_eq!(grid.get(Coord::new(2, 1)), Some(&true)); // bottom
        assert_eq!(grid.get(Coord::new(1, 2)), Some(&true)); // left
        assert_eq!(grid.get(Coord::new(2, 3)), Some(&true)); // top
    }

    #[test]
    fn selection_iter_mut_already_visited() {
        let mut grid: VecGrid<bool> = VecGrid::new(Rect::new((3, 3)));
        let coords = [(2, 2), (2, 2)].iter().map(|&x| x.into());
        let mut iter = grid.selection_iter_mut(coords);
        assert!(iter.next().unwrap().is_ok());
        assert!(iter.next().unwrap() == Err(GridError::AlreadyVisited(Coord::new(2, 2))));
    }
}
