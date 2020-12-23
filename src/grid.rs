use std::{
    collections::{HashSet, VecDeque},
    fmt, mem,
};

use crate::{
    coord::Coord,
    patterns::{self, RectBounds},
};

/// The core type of this library. A 2D grid of cell type `T`.
pub struct Grid<T> {
    /// Row-major, linear storage of cell data.
    pub cells: Vec<T>,
    /// XY dimensions of the grid. This uses a [`Coord`](crate::coord::Coord) for
    /// consistency with the rest of the code and to reduce the amount of type
    /// casting. Components must be greater than zero.
    pub dimensions: Coord,
    pub offset: Coord,
}

impl<T> Grid<T> {
    pub fn new<D, O>(dimensions: D, offset: O) -> Self
    where
        T: Default + Clone,
        D: Into<Coord>,
        O: Into<Coord>,
    {
        let dimensions = dimensions.into();
        assert!(
            dimensions.x > 0 && dimensions.y > 0,
            format!("Grid dimensions must be positive: {}", dimensions)
        );
        Self {
            cells: vec![T::default(); (dimensions.x * dimensions.y) as usize],
            dimensions,
            offset: offset.into(),
        }
    }

    pub fn with_generator<D, O, C>(dimensions: D, offset: O, generator: impl Fn(C) -> T) -> Self
    where
        D: Into<Coord>,
        O: Into<Coord>,
        C: From<Coord>,
    {
        let dimensions = dimensions.into();
        assert!(
            dimensions.x > 0 && dimensions.y > 0,
            format!("Grid dimensions must be positive: {}", dimensions)
        );
        let offset = offset.into();
        let mut cells = Vec::with_capacity((dimensions.x * dimensions.y) as usize);
        // TODO: Implement an iterator over all grid cells.
        for y in offset.y..(dimensions.y + offset.y) {
            for x in offset.x..(dimensions.x + offset.x) {
                let coord = Coord::new(x, y);
                cells.push(generator(coord.into()));
            }
        }
        Self {
            cells,
            dimensions,
            offset,
        }
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.cells.get(self.coord_to_index(coord)?)
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let index = self.coord_to_index(coord)?;
        self.cells.get_mut(index)
    }

    pub fn set(&mut self, coord: Coord, value: T) -> bool {
        if let Some(cell) = self.get_mut(coord) {
            *cell = value;
            true
        } else {
            false
        }
    }

    pub fn replace(&mut self, coord: Coord, value: T) -> Option<T> {
        self.get_mut(coord)
            .and_then(|cell| Some(mem::replace(cell, value)))
    }

    pub fn take(&mut self, coord: Coord) -> Option<T>
    where
        T: Default,
    {
        self.get_mut(coord).and_then(|cell| Some(mem::take(cell)))
    }

    pub fn copy(&mut self, src: Coord, dest: Coord) -> bool
    where
        T: Copy,
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
    pub fn swap(&mut self, coord1: Coord, coord2: Coord) -> bool {
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
    pub fn mov(&mut self, src: Coord, dest: Coord) -> Option<T>
    where
        T: Default,
    {
        // Make sure both coordinates are in bounds before mutating things.
        if !(self.coord_in_bounds(src) && self.coord_in_bounds(dest)) {
            return None;
        }
        let src_value = self.take(src).unwrap();
        self.replace(dest, src_value)
    }

    /// Returns an iterator over all cells in the grid.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Coord, &'a T)> {
        self.cells
            .iter()
            .enumerate()
            .map(move |cell| (self.index_to_coord(cell.0), cell.1))
    }

    pub fn cell_iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.cells.iter_mut()
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
    pub fn flood_iter(
        &self,
        starting_coord: Coord,
        predicate: impl Fn(&T) -> bool + 'static,
    ) -> FloodIter<T> {
        let mut coords_to_search = VecDeque::new();
        coords_to_search.push_back(starting_coord);

        FloodIter {
            grid: self,
            predicate: Box::new(predicate),
            searched_coords: vec![],
            coords_to_search,
        }
    }

    fn coord_to_index(&self, coord: Coord) -> Option<usize> {
        if !self.coord_in_bounds(coord) {
            return None;
        }
        let offset_coord = coord - self.offset;
        Some((offset_coord.x + offset_coord.y * self.dimensions.x) as usize)
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        let y = (index as f32 / self.dimensions.y as f32).floor() as i32;
        let x = index as i32 - (y * self.dimensions.y) as i32;
        Coord::new(x, y) + self.offset
    }

    fn bounds(&self) -> RectBounds {
        RectBounds {
            top: self.dimensions.y - self.offset.y - 1,
            bottom: self.offset.y,
            left: self.offset.x,
            right: self.dimensions.x - self.offset.x - 1,
        }
    }

    fn coord_in_bounds(&self, coord: Coord) -> bool {
        self.bounds().contains(coord)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    /// The coordinate has no cell associated with it, as it's out of the grid
    /// bounds.
    OutOfBounds(Coord),
    /// The coordinate has previously been mutably borrowed from the iterator,
    /// and doing so again would break safety guarantees.
    AlreadyVisited(Coord),
}

pub struct SelectionIter<'a, T, I> {
    grid: &'a Grid<T>,
    coords: I,
}

impl<'a, T, I> Iterator for SelectionIter<'a, T, I>
where
    I: Iterator<Item = Coord>,
{
    type Item = Result<(Coord, &'a T), GridError>;

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
    grid: &'a mut Grid<T>,
    coords: I,
    visited_coords: HashSet<Coord>,
}

impl<'a, T, I> Iterator for SelectionIterMut<'a, T, I>
where
    I: Iterator<Item = Coord>,
{
    type Item = Result<(Coord, &'a mut T), GridError>;

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
    grid: &'a Grid<T>,
    predicate: Box<dyn Fn(&T) -> bool>,
    searched_coords: Vec<Coord>,
    coords_to_search: VecDeque<Coord>,
}

impl<'a, T> Iterator for FloodIter<'a, T> {
    type Item = (Coord, &'a T);

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

            let neighbor_coords = patterns::ortho_neighborhood(coord)
                .filter(|&coord| {
                    !(self.searched_coords.contains(&coord)
                        || self.coords_to_search.contains(&coord))
                        && self.grid.coord_in_bounds(coord)
                })
                .collect::<Vec<Coord>>();

            self.coords_to_search.extend(neighbor_coords);

            return Some((coord, self.grid.get(coord).unwrap()));
        }

        None
    }
}

impl<T> fmt::Display for Grid<T>
where
    char: From<T>,
    T: Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.offset.y..(self.dimensions.y + self.offset.y) {
            for x in self.offset.x..(self.dimensions.x + self.offset.x) {
                let coord = Coord::new(x, y);
                let c: char = match self.get(coord) {
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
    fn out_of_bounds_coords() {
        let grid = Grid::<()>::new((8, 8), (0, 0));
        // This would pass `coord_to_index` if there was no bounds check.
        assert_eq!(grid.get(Coord::new(-1, 4)), None);
    }

    #[test]
    fn selection_iter_mut() {
        let mut grid: Grid<bool> = Grid::new((4, 4), (0, 0));
        // Set all neighbors of (2, 2) to `true`.
        for res_cell in grid.selection_iter_mut(patterns::neighborhood((2, 2))) {
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
        let mut grid: Grid<bool> = Grid::new((4, 4), (0, 0));
        let coords = [(2, 2), (2, 2)].iter().map(|&x| x.into());
        let mut iter = grid.selection_iter_mut(coords);
        assert!(iter.next().unwrap().is_ok());
        assert!(iter.next().unwrap() == Err(GridError::AlreadyVisited(Coord::new(2, 2))));
    }
}
