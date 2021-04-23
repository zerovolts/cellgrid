use tapestry::{
    patterns::{Circle, Rect},
    VecGrid,
};

fn main() {
    let mut grid: VecGrid<char> = VecGrid::new(Rect::new((19, 19)));

    // Fill grid background
    for (_coord, cell) in grid.iter_mut() {
        *cell = '∙';
    }

    let external_circle = Circle::new((9, 9), 7);
    let internal_circle = Circle::new((9, 9), 3);
    let ring_coords = external_circle.iter().chain(internal_circle.iter());
    // Draw ring outline
    for result in grid.selection_iter_mut(ring_coords) {
        if let Ok((_coord, cell)) = result {
            *cell = '#';
        }
    }

    let flood_coords = grid
        .flood_iter((6, 6), |&cell| cell != '#')
        .map(|(coord, _cell)| coord)
        // `collect` to release the borrow on `grid`.
        .collect::<Vec<_>>()
        .into_iter();
    // Fill ring
    for result in grid.selection_iter_mut(flood_coords) {
        if let Ok((_coord, cell)) = result {
            *cell = '/';
        }
    }

    println!("{}", grid);
}
