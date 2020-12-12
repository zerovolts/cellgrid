use std::{collections::VecDeque, mem};

use crate::coord::Coord;

pub struct Grid<T> {
    // Row-major, linear storage of 2d grid cells.
    pub cells: Vec<T>,
    pub dimensions: (u32, u32),
    pub offset: Coord,
}

impl<T> Grid<T> {
    pub fn new<O: Into<Coord>>(dimensions: (u32, u32), offset: O) -> Self
    where
        T: Default + Clone,
    {
        Self {
            cells: vec![T::default(); (dimensions.0 * dimensions.1) as usize],
            dimensions,
            offset: offset.into(),
        }
    }

    pub fn with_generator<O: Into<Coord>, C: From<Coord>>(
        dimensions: (u32, u32),
        offset: O,
        generator: impl Fn(C) -> T,
    ) -> Self {
        let mut cells = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
        // TODO: Implement an iterator over all grid cells.
        for y in 0..(dimensions.1 as i32) {
            for x in 0..(dimensions.0 as i32) {
                let coord = Coord(x, y);
                cells.push(generator(coord.into()));
            }
        }
        Self {
            cells,
            dimensions,
            offset: offset.into(),
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

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Coord, &'a T)> {
        self.cells
            .iter()
            .enumerate()
            .map(move |cell| (self.index_to_coord(cell.0), cell.1))
    }

    pub fn cell_iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.cells.iter_mut()
    }

    pub fn offset_iter(&self, starting_point: Coord, offsets: &[Coord]) -> SelectionIter<T> {
        SelectionIter {
            grid: self,
            coords: starting_point
                .anchor_coords(offsets)
                .collect::<VecDeque<Coord>>(),
        }
    }

    pub fn selection_iter(&self, coords: &[Coord]) -> SelectionIter<T> {
        let coords_vec: Vec<_> = coords.into();
        SelectionIter {
            grid: self,
            coords: coords_vec.into(),
        }
    }

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
        (offset_coord.0 + offset_coord.1 * self.dimensions.0 as i32) as usize
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        let y = (index as f32 / self.dimensions.1 as f32).floor() as i32;
        let x = index as i32 - (y * self.dimensions.1 as i32) as i32;
        Coord(x + self.offset.0, y + self.offset.1)
    }

    fn max_x(&self) -> i32 {
        self.dimensions.0 as i32 - self.offset.0
    }

    fn max_y(&self) -> i32 {
        self.dimensions.1 as i32 - self.offset.1
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
pub struct SelectionIter<'a, T> {
    grid: &'a Grid<T>,
    coords: VecDeque<Coord>,
}

impl<'a, T> Iterator for SelectionIter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        // Loop in case some of the offsets are invalid
        while self.coords.len() > 0 {
            let coord = self.coords.pop_front().unwrap();
            if let Some(cell) = self.grid.get(coord) {
                return Some((coord, cell));
            } else {
                continue;
            }
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

            let neighbor_coords = coord
                .ortho_neighbor_coords()
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
