use crate::coord::Coord;

pub struct Rect {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Rect {
    pub fn contains(&self, coord: Coord) -> bool {
        coord.x >= self.left
            && coord.x <= self.right
            && coord.y >= self.bottom
            && coord.y <= self.top
    }
}

pub fn rect(from_corner: Coord, to_corner: Coord) -> impl Iterator<Item = Coord> {
    let rect = Rect {
        top: from_corner.y.max(to_corner.y),
        bottom: from_corner.y.min(to_corner.y),
        left: from_corner.x.min(to_corner.x),
        right: from_corner.x.max(to_corner.x),
    };
    let next_coord = Coord::new(rect.left, rect.bottom);

    RectIter {
        rect,
        next_coord,
        is_finished: false,
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
        let coord_count = rect(Coord::new(0, 0), Coord::new(0, 0)).count();
        assert_eq!(coord_count, 1);
    }

    #[test]
    fn multi_coord_rect_iter() {
        let coord_count = rect(Coord::new(0, 0), Coord::new(3, 3)).count();
        assert_eq!(coord_count, 16);
    }

    #[test]
    fn reversed_coord_rect_iter() {
        let coords = rect(Coord::new(2, 2), Coord::new(-2, -2)).collect::<Vec<_>>();
        assert_eq!(coords.first(), Some(&Coord::new(-2, -2)));
        assert_eq!(coords.last(), Some(&Coord::new(2, 2)));
    }
}
