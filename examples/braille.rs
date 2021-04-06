use tenji::{braille::braillify_grid, coord::Coord, grid::Grid, patterns, rect::Rect};

fn main() {
    let mut bitgrid = Grid::<bool>::new(Rect::with_corners((0, 0), (60, 50)));

    // Draw the outline of a shape.

    let c1 = Coord::new(27, 3);
    let c2 = Coord::new(52, 12);
    let c3 = Coord::new(50, 42);
    let c4 = Coord::new(20, 45);
    let c5 = Coord::new(5, 20);

    let coords = patterns::line(c1, c2)
        .chain(patterns::line(c2, c3))
        .chain(patterns::line(c3, c4))
        .chain(patterns::line(c4, c5))
        .chain(patterns::line(c5, c1));

    for result in bitgrid.selection_iter_mut(coords) {
        if let Ok((_coord, cell)) = result {
            *cell = true;
        }
    }

    // Fill the interior of the shape.

    let flood_coords = bitgrid
        .flood_iter(Coord::new(32, 32), |&cell| cell == false)
        .map(|(coord, _)| coord)
        .collect::<Vec<Coord>>();

    for result in bitgrid.selection_iter_mut(flood_coords.iter().map(|&coord| coord)) {
        if let Ok((_coord, cell)) = result {
            *cell = true;
        }
    }

    println!("{}", braillify_grid(bitgrid));
}
