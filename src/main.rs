use grid::{Coord, Grid};

mod grid;

fn main() {
    let mut grid = Grid::new((16, 16), (-8, -8));
    for cell in grid.cells.iter_mut() {
        *cell = rand::random::<bool>();
    }
    println!("cells: {:?}", grid.cells);
    println!(
        "flood: {:?}",
        grid.flood_iter(Coord(0, 0), |&cell| cell == true)
            .collect::<Vec<(Coord, &bool)>>()
    );
}
