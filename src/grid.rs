use std::{
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

    fn coord_to_index(&self, coord: Coord) -> usize {
        let offset_coord = coord - self.offset;
        (offset_coord.0 + offset_coord.1 * self.dimensions.0 as i32) as usize
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
