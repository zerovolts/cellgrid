use crate::coord::Coord;

#[derive(Clone, Copy)]
pub struct Rect {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Rect {
    pub fn with_corners(corner1: Coord, corner2: Coord) -> Self {
        Self {
            top: corner1.y.max(corner2.y),
            bottom: corner1.y.min(corner2.y),
            left: corner1.x.min(corner2.x),
            right: corner1.x.max(corner2.x),
        }
    }

    pub fn contains(&self, coord: Coord) -> bool {
        coord.x >= self.left
            && coord.x <= self.right
            && coord.y >= self.bottom
            && coord.y <= self.top
    }

    pub fn iter(&self) -> impl Iterator<Item = Coord> {
        let next_coord = Coord::new(self.left, self.bottom);

        RectIter {
            rect: *self,
            next_coord,
            is_finished: false,
        }
    }
}

/// Iterates row by row from the bottom-left corner to the top-right corner.
struct RectIter {
    rect: Rect,
    next_coord: Coord,
    is_finished: bool,
}

impl Iterator for RectIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        // We return the coordinate computed on the previous iteration.
        let next_coord = self.next_coord;

        if self.next_coord.x < self.rect.right {
            self.next_coord.x += 1;
        } else if self.next_coord.y < self.rect.top {
            self.next_coord.x = self.rect.left;
            self.next_coord.y += 1;
        } else {
            // We've passed the last element.
            self.is_finished = true;
        }

        Some(next_coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_coord_rect_iter() {
        let rect = Rect::with_corners(Coord::new(0, 0), Coord::new(0, 0));
        assert_eq!(rect.iter().count(), 1);
    }

    #[test]
    fn multi_coord_rect_iter() {
        let rect = Rect::with_corners(Coord::new(0, 0), Coord::new(3, 3));
        assert_eq!(rect.iter().count(), 16);
    }

    #[test]
    fn reversed_coord_rect_iter() {
        let rect = Rect::with_corners(Coord::new(2, 2), Coord::new(-2, -2));
        let coords = rect.iter().collect::<Vec<_>>();
        assert_eq!(coords.first(), Some(&Coord::new(-2, -2)));
        assert_eq!(coords.last(), Some(&Coord::new(2, 2)));
    }
}
