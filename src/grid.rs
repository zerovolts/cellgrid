use std::{
    collections::{HashSet, VecDeque},
    fmt, mem,
};

use crate::{coord::Coord, patterns};

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
    pub fn new<C: Into<Coord>>(dimensions: C, offset: C) -> Self
    where
        T: Default + Clone,
    {
        let dimensions = dimensions.into();
        assert!(
            dimensions.0 > 0 && dimensions.1 > 0,
            format!("Grid dimensions must be positive: {}", dimensions)
        );
        Self {
            cells: vec![T::default(); (dimensions.0 * dimensions.1) as usize],
            dimensions,
            offset: offset.into(),
        }
    }

    pub fn with_generator<C: Into<Coord>, Fc: From<Coord>>(
        dimensions: C,
        offset: C,
        generator: impl Fn(Fc) -> T,
    ) -> Self {
        let dimensions = dimensions.into();
        assert!(
            dimensions.0 > 0 && dimensions.1 > 0,
            format!("Grid dimensions must be positive: {}", dimensions)
        );
        let mut cells = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
        let offset = offset.into();
        // TODO: Implement an iterator over all grid cells.
        for y in offset.1..(dimensions.1 + offset.1) {
            for x in offset.0..(dimensions.0 + offset.0) {
                let coord = Coord(x, y);
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
        self.cells.get(self.coord_to_index(coord))
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let index = self.coord_to_index(coord);
        self.cells.get_mut(index)
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        if let Some(cell) = self.get_mut(coord) {
            *cell = value;
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

    pub fn copy(&mut self, src: Coord, dest: Coord)
    where
        T: Copy,
    {
        let src_index = self.coord_to_index(src);
        let dest_index = self.coord_to_index(dest);
        self.cells
            .copy_within(src_index..(src_index + 1), dest_index)
    }

    /// Swaps the contents of two cells.
    pub fn swap(&mut self, coord1: Coord, coord2: Coord) {
        let index1 = self.coord_to_index(coord1);
        let index2 = self.coord_to_index(coord2);
        self.cells.swap(index1, index2);
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

    fn coord_to_index(&self, coord: Coord) -> usize {
        let offset_coord = coord - self.offset;
        (offset_coord.0 + offset_coord.1 * self.dimensions.0) as usize
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        let y = (index as f32 / self.dimensions.1 as f32).floor() as i32;
        let x = index as i32 - (y * self.dimensions.1) as i32;
        Coord(x, y) + self.offset
    }

    fn max_x(&self) -> i32 {
        self.dimensions.0 - self.offset.0
    }

    fn max_y(&self) -> i32 {
        self.dimensions.1 - self.offset.1
    }

    fn min_x(&self) -> i32 {
        self.offset.0
    }

    fn min_y(&self) -> i32 {
        self.offset.1
    }

    fn coord_in_bounds(&self, coord: Coord) -> bool {
        coord.0 < self.max_x()
            && coord.0 >= self.min_x()
            && coord.0 < self.max_y()
            && coord.0 >= self.min_y()
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

            let neighbor_coords = patterns::ortho_neighbor_coords()
                .map(|offset| coord + offset)
                .filter(|&coord| {
                    !(self.searched_coords.contains(&coord)
                        || self.coords_to_search.contains(&coord))
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
        for y in self.offset.1..(self.dimensions.1 + self.offset.1) {
            for x in self.offset.0..(self.dimensions.0 + self.offset.0) {
                let coord = Coord(x, y);
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
    fn selection_iter_mut() {
        let mut grid: Grid<bool> = Grid::new((4, 4), (0, 0));
        // Set all neighbors of (2, 2) to `true`.
        for res_cell in
            grid.selection_iter_mut(patterns::neighbor_coords().map(|coord| coord + Coord(2, 2)))
        {
            *res_cell.unwrap().1 = true;
        }
        assert_eq!(grid.get(Coord(2, 2)), Some(&false)); // center
        assert_eq!(grid.get(Coord(3, 2)), Some(&true)); // right
        assert_eq!(grid.get(Coord(2, 1)), Some(&true)); // bottom
        assert_eq!(grid.get(Coord(1, 2)), Some(&true)); // left
        assert_eq!(grid.get(Coord(2, 3)), Some(&true)); // top
    }

    #[test]
    fn selection_iter_mut_already_visited() {
        let mut grid: Grid<bool> = Grid::new((4, 4), (0, 0));
        let coords = [Coord(2, 2), Coord(2, 2)].iter().map(|&x| x);
        let mut iter = grid.selection_iter_mut(coords);
        assert!(iter.next().unwrap().is_ok());
        assert!(iter.next().unwrap() == Err(GridError::AlreadyVisited(Coord(2, 2))));
    }
}
