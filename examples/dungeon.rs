use rand::Rng;

use tapestry::{
    patterns::{Cluster, Orientation, Rect},
    VecGrid,
};

fn main() {
    let rect = Rect::new((64, 64));
    // The minimum distance a partition can get to the edge of a Rect.
    let min_size = 8;

    let mut grid = VecGrid::<Tile>::new(rect);
    let room_tree = grid
        .bounds
        .bsp(Orientation::Horizontal, &|rect, orientation| {
            let dimension = match orientation {
                Orientation::Horizontal => rect.width(),
                Orientation::Vertical => rect.height(),
            };
            let max_size = dimension - min_size;
            // No valid partitions; any cut would make the leaves too small.
            if max_size - min_size <= 0 {
                return None;
            }
            let partition = rand::thread_rng().gen_range(min_size..=max_size);
            Some((partition, orientation.orthogonal()))
        });
    let room_bounds = room_tree.leaves();
    let rooms = room_bounds.iter().flat_map(|room| {
        shrink_randomly(
            Rect {
                bottom: room.bottom,
                left: room.left + 1,
                top: room.top + 1,
                right: room.right,
            },
            4,
        )
        .iter()
    });

    // room_tree

    let cluster = Cluster::new(rooms);
    for result in grid.selection_iter_mut(cluster.iter_interior()) {
        if let Ok((_coord, cell)) = result {
            *cell = Tile::Floor;
        }
    }
    for result in grid.selection_iter_mut(cluster.iter_internal_border()) {
        if let Ok((_coord, cell)) = result {
            *cell = Tile::Wall;
        }
    }
    println!("{}", grid);
}

fn shrink_randomly(rect: Rect, min_dimension: i32) -> Rect {
    if min_dimension >= rect.width() || min_dimension >= rect.height() {
        return rect;
    }
    let horizontal_shrink = rand::thread_rng().gen_range(0..rect.width() - min_dimension);
    let vertical_shrink = rand::thread_rng().gen_range(0..rect.height() - min_dimension);
    let new_x = if horizontal_shrink > 0 {
        rand::thread_rng().gen_range(0..horizontal_shrink)
    } else {
        0
    };
    let new_y = if vertical_shrink > 0 {
        rand::thread_rng().gen_range(0..vertical_shrink)
    } else {
        0
    };
    Rect {
        bottom: rect.bottom - vertical_shrink,
        right: rect.right - horizontal_shrink,
        ..rect
    }
    .translate((new_x, new_y))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Void,
    Wall,
    Floor,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Void
    }
}

impl From<Tile> for char {
    fn from(ls: Tile) -> char {
        match ls {
            Tile::Void => ' ',
            Tile::Wall => '#',
            Tile::Floor => '∙',
        }
    }
}
