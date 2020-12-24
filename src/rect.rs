use std::ops::Range;

use crate::coord::Coord;

#[derive(Clone, Copy)]
pub struct Rect {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Rect {
    /// Returns a RectBounds, given any two (inclusive) corners of a rectangle.
    ///
    /// It is advisable to use this over creating a RectBounds literal, because
    /// this will prevent invalid states, such as `top` being less than `bottom`.
    pub fn with_corners<C1, C2>(corner1: C1, corner2: C2) -> Self
    where
        C1: Into<Coord>,
        C2: Into<Coord>,
    {
        let corner1 = corner1.into();
        let corner2 = corner2.into();
        Self {
            top: corner1.y.max(corner2.y),
            bottom: corner1.y.min(corner2.y),
            left: corner1.x.min(corner2.x),
            right: corner1.x.max(corner2.x),
        }
    }

    pub fn dimensions(&self) -> Coord {
        Coord::new(self.width(), self.height())
    }

    pub fn offset(&self) -> Coord {
        Coord::new(self.left, self.bottom)
    }

    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn width(&self) -> i32 {
        // Add one because `right` is inclusive.
        (self.right - self.left) + 1
    }

    pub fn height(&self) -> i32 {
        // Add one because `top` is inclusive.
        (self.top - self.bottom) + 1
    }

    pub fn contains(&self, coord: Coord) -> bool {
        coord.x >= self.left
            && coord.x <= self.right
            && coord.y >= self.bottom
            && coord.y <= self.top
    }

    pub fn x_range(&self) -> Range<i32> {
        // Add one because `right` is inclusive.
        self.left..(self.right + 1)
    }

    pub fn y_range(&self) -> Range<i32> {
        // Add one because `top` is inclusive.
        self.bottom..(self.top + 1)
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
    fn dimensions() {
        let rect = Rect::with_corners(Coord::new(0, 0), Coord::new(3, 4));
        assert_eq!(rect.width(), 4);
        assert_eq!(rect.height(), 5);
    }

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
