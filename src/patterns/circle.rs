use std::collections::{HashSet, VecDeque};

use crate::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Circle {
    pub center: Coord,
    pub radius: i32,
}

impl Circle {
    pub fn new<C: Into<Coord>>(center: C, radius: i32) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }

    /// Traces Bresenham's circle algorithm.
    pub fn iter(&self) -> CircleIter {
        let mut coord_queue = VecDeque::new();
        let mut seen_coords = HashSet::new();

        let starting_coord = Coord::new(0, self.radius);

        for coord in self
            .mirror_quadrants(starting_coord)
            .iter()
            .map(|&coord| coord)
        {
            coord_queue.push_back(coord);
            seen_coords.insert(coord);
        }

        CircleIter {
            circle: *self,
            cursor: starting_coord,
            d: 3 - (2 * self.radius),
            coord_queue,
            seen_coords,
        }
    }

    fn mirror_quadrants(&self, coord: Coord) -> [Coord; 4] {
        [
            self.center + coord,
            self.center + coord.flip(),
            self.center + coord.negate_y(),
            self.center + coord.flip().negate(),
        ]
    }

    fn mirror_octants(&self, coord: Coord) -> [Coord; 8] {
        [
            self.center + coord,
            self.center + coord.flip(),
            self.center + coord.flip().negate_x(),
            self.center + coord.negate_x(),
            self.center + coord.negate(),
            self.center + coord.flip().negate(),
            self.center + coord.flip().negate_y(),
            self.center + coord.negate_y(),
        ]
    }
}

pub struct CircleIter {
    circle: Circle,
    cursor: Coord,
    d: i32,
    /// Coords to be returned on subsequent iterations.
    coord_queue: VecDeque<Coord>,
    /// Used to prevent duplicate Coords from being returned.
    seen_coords: HashSet<Coord>,
}

impl Iterator for CircleIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord_queue.len() == 0 && self.cursor.y > self.cursor.x {
            self.cursor.x += 1;

            if self.d < 0 {
                self.d = self.d + (4 * self.cursor.x) + 6;
            } else {
                self.d = 4 * (self.cursor.x - self.cursor.y) + 10;
                self.cursor.y -= 1;
            }

            for coord in self.circle.mirror_octants(self.cursor).iter() {
                if !self.seen_coords.contains(coord) {
                    self.seen_coords.insert(*coord);
                    self.coord_queue.push_back(*coord);
                }
            }
        }

        if self.coord_queue.len() > 0 {
            return self.coord_queue.pop_front();
        }

        None
    }
}
