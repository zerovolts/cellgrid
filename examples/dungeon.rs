use rand::Rng;

use tenji::{
    grid::Grid,
    patterns::cluster_layers,
    rect::{Orientation, Rect},
};

fn main() {
    let rect = Rect::new((64, 64));
    // The minimum distance a partition can get to the edge of a Rect.
    let min_size = 8;

    let mut grid = Grid::<LifeState>::new(rect);
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
                bottom: room.bottom + 1,
                left: room.left + 1,
                top: room.top,
                right: room.right,
            },
            4,
        )
        .iter()
    });

    // room_tree

    let layers = cluster_layers(rooms.collect());
    for result in grid.selection_iter_mut(layers.interior.into_iter()) {
        if let Ok((_coord, cell)) = result {
            *cell = LifeState::Floor;
        }
    }
    for result in grid.selection_iter_mut(layers.internal_border.into_iter()) {
        if let Ok((_coord, cell)) = result {
            *cell = LifeState::Wall;
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
        top: rect.top - vertical_shrink,
        right: rect.right - horizontal_shrink,
        ..rect
    }
    .translate((new_x, new_y))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LifeState {
    Void,
    Wall,
    Floor,
}

impl Default for LifeState {
    fn default() -> Self {
        LifeState::Void
    }
}

impl From<LifeState> for char {
    fn from(ls: LifeState) -> char {
        match ls {
            LifeState::Void => ' ',
            LifeState::Wall => '■',
            LifeState::Floor => '∙',
        }
    }
}
