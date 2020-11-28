use std::{
    collections::VecDeque,
    mem,
    ops::{Add, Sub},
};

pub struct Grid<T> {
    // Row-major, linear storage of 2d grid cells.
    pub cells: Vec<T>,
    pub dimensions: (u32, u32),
    pub offset: Coord,
}

impl<T> Grid<T> {
    pub fn new(dimensions: (u32, u32), offset: (i32, i32), value: T) -> Self
    where
        T: Clone,
    {
        Self {
            cells: vec![value; (dimensions.0 * dimensions.1) as usize],
            dimensions,
            offset: Coord(offset.0, offset.1),
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
        let index = self.coord_to_index(coord);
        if let Some(cell) = self.cells.get_mut(index) {
            *cell = value;
        }
    }

    pub fn replace(&mut self, coord: Coord, value: T) -> Option<T> {
        let index = self.coord_to_index(coord);
        self.cells
            .get_mut(index)
            .and_then(|cell| Some(mem::replace(cell, value)))
    }

    pub fn offset_iter(&self, starting_point: Coord, offsets: &[Coord]) -> SelectionIter<T> {
        SelectionIter {
            grid: self,
            coords: Self::offsets_to_coords(starting_point, offsets).into(),
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

    pub fn ortho_neighbor_coords(&self, coord: Coord) -> Vec<Coord> {
        Self::offsets_to_coords(
            coord,
            &[Coord(1, 0), Coord(0, -1), Coord(-1, 0), Coord(0, 1)],
        )
    }

    fn offsets_to_coords(coord: Coord, offsets: &[Coord]) -> Vec<Coord> {
        offsets.iter().map(|&offset| coord + offset).collect()
    }

    fn coord_to_index(&self, coord: Coord) -> usize {
        let offset_coord = coord - self.offset;
        (offset_coord.0 + offset_coord.1 * self.dimensions.0 as i32) as usize
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

            let neighbor_coords = self
                .grid
                .ortho_neighbor_coords(coord)
                .iter()
                .filter(|&coord| {
                    !(self.searched_coords.contains(coord) || self.coords_to_search.contains(coord))
                })
                .map(|&coord| coord)
                .collect::<Vec<Coord>>();

            self.coords_to_search.extend(neighbor_coords);

            return Some((coord, self.grid.get(coord).unwrap()));
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub i32, pub i32);

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}
