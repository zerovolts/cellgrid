use std::ops::Range;

use crate::coord::Coord;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Rect {
    /// Constructs a Rect at (0, 0).
    pub fn new<C: Into<Coord>>(dimensions: C) -> Self {
        Self::with_corners((0, 0), dimensions)
    }

    /// Constructs a Rect, given _any_ two corners.
    ///
    /// It is advisable to use this over creating a RectBounds literal, because
    /// this will prevent invalid states, such as `left` being less than `right`.
    pub fn with_corners<C1, C2>(corner1: C1, corner2: C2) -> Self
    where
        C1: Into<Coord>,
        C2: Into<Coord>,
    {
        let corner1 = corner1.into();
        let corner2 = corner2.into();
        Self {
            // Top is the less than bottom to match other graphics applications.
            top: corner1.y.min(corner2.y),
            bottom: corner1.y.max(corner2.y),
            left: corner1.x.min(corner2.x),
            right: corner1.x.max(corner2.x),
        }
    }

    pub fn dimensions(&self) -> Coord {
        Coord::new(self.width(), self.height())
    }

    pub fn offset(&self) -> Coord {
        Coord::new(self.left, self.top)
    }

    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn partition_vertical(&self, partition: i32) -> (Self, Self) {
        let absolute_partition = self.top + partition;
        (
            // Bottom partition (physically top)
            Self {
                top: absolute_partition,
                ..*self
            },
            // Top partition (physically bottom)
            Self {
                bottom: absolute_partition,
                ..*self
            },
        )
    }

    pub fn partition_horizontal(&self, partition: i32) -> (Self, Self) {
        let absolute_partition = self.left + partition;
        (
            // Left partition
            Self {
                right: absolute_partition,
                ..*self
            },
            // Right partition
            Self {
                left: absolute_partition,
                ..*self
            },
        )
    }

    /// Executes a binary space partition with the given splitter algorithm.
    pub fn bsp(
        &self,
        orientation: Orientation,
        splitter: &dyn Fn(Rect, Orientation) -> Option<(i32, Orientation)>,
    ) -> BspTree {
        match splitter(*self, orientation) {
            Some((partition, next_orientation)) => {
                let (left_or_bottom, right_or_top) = match orientation {
                    Orientation::Horizontal => self.partition_horizontal(partition),
                    Orientation::Vertical => self.partition_vertical(partition),
                };
                BspTree::Node(
                    *self,
                    Box::new(left_or_bottom.bsp(next_orientation, splitter)),
                    Box::new(right_or_top.bsp(next_orientation, splitter)),
                )
            }
            None => BspTree::Leaf(*self),
        }
    }

    pub fn translate<C: Into<Coord>>(&self, coord: C) -> Self {
        let coord = coord.into();
        Self {
            bottom: self.bottom + coord.y,
            left: self.left + coord.x,
            top: self.top + coord.y,
            right: self.right + coord.x,
        }
    }

    pub fn contains<C: Into<Coord>>(&self, coord: C) -> bool {
        let coord = coord.into();
        coord.x >= self.left && coord.x < self.right && coord.y >= self.top && coord.y < self.bottom
    }

    pub fn x_range(&self) -> Range<i32> {
        self.left..self.right
    }

    pub fn y_range(&self) -> Range<i32> {
        self.top..self.bottom
    }

    pub fn iter(&self) -> impl Iterator<Item = Coord> {
        let next_coord = Coord::new(self.left, self.top);

        RectIter {
            rect: *self,
            next_coord,
            is_finished: false,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn orthogonal(&self) -> Self {
        match self {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal,
        }
    }
}

#[derive(Debug)]
pub enum BspTree {
    Node(Rect, Box<BspTree>, Box<BspTree>),
    Leaf(Rect),
}

impl BspTree {
    pub fn leaves(&self) -> Vec<Rect> {
        match self {
            BspTree::Node(_, left, right) => {
                let mut leaves = left.leaves();
                leaves.append(&mut right.leaves());
                leaves
            }
            BspTree::Leaf(rect) => vec![*rect],
        }
    }
}

/// Iterates row by row from the bottom-left corner to the top-right corner.
pub struct RectIter {
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

        if self.next_coord.x < self.rect.right - 1 {
            self.next_coord.x += 1;
        } else if self.next_coord.y < self.rect.bottom - 1 {
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
        let rect = Rect::new((3, 4));
        assert_eq!(rect.width(), 3);
        assert_eq!(rect.height(), 4);
    }

    #[test]
    fn single_coord_rect_iter() {
        let rect = Rect::new((0, 0));
        assert_eq!(rect.iter().count(), 1);
    }

    #[test]
    fn multi_coord_rect_iter() {
        let rect = Rect::new((4, 4));
        assert_eq!(rect.iter().count(), 16);
    }

    #[test]
    fn reversed_coord_rect_iter() {
        let rect = Rect::with_corners((3, 3), (-2, -2));
        let coords = rect.iter().collect::<Vec<_>>();
        assert_eq!(coords.first(), Some(&Coord::new(-2, -2)));
        assert_eq!(coords.last(), Some(&Coord::new(2, 2)));
    }

    #[test]
    fn vertical_partitioning() {
        let rect = Rect::new((8, 8));
        let (top, bottom) = rect.partition_vertical(4);
        assert_eq!(top.area(), 32);
        assert_eq!(bottom.area(), 32);
    }

    #[test]
    fn vertical_zero_partitioning() {
        let rect = Rect::new((8, 8));
        let (bottom, top) = rect.partition_vertical(0);
        assert_eq!(top.area(), 0);
        assert_eq!(bottom.area(), 64);
    }

    #[test]
    fn horizontal_partitioning() {
        let rect = Rect::new((8, 8));
        let (left, right) = rect.partition_horizontal(4);
        assert_eq!(left.area(), 32);
        assert_eq!(right.area(), 32);
    }

    #[test]
    fn horizontal_zero_partitioning() {
        let rect = Rect::new((8, 8));
        let (left, right) = rect.partition_horizontal(0);
        assert_eq!(left.area(), 0);
        assert_eq!(right.area(), 64);
    }

    #[test]
    fn equally_subdivided_bsp() {
        let rect = Rect::new((16, 16));
        // The minimum distance a partition can get to the edge of a Rect.
        let min_size = 4;

        // Subdivide the rect into 16 4x4 pieces.
        let bsp_leaves = rect
            .bsp(Orientation::Horizontal, &|rect, orientation| {
                let partition = match orientation {
                    Orientation::Horizontal => rect.width() / 2,
                    Orientation::Vertical => rect.height() / 2,
                };
                if partition < min_size {
                    return None;
                }
                Some((partition, orientation.orthogonal()))
            })
            .leaves();

        assert_eq!(bsp_leaves.len(), 16);
        assert!(bsp_leaves.iter().all(|rect| rect.area() == 16));
    }
}
