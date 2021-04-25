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

    let circle = Circle::new((9, 9), 7);
    // Draw circle
    for result in grid.selection_iter_mut(circle.iter()) {
        if let Ok((_coord, cell)) = result {
            *cell = '#';
        }
    }

    // Map grid of characters into a grid of strings, addings spaces between the
    // characters.
    let display_grid = grid.map(|cell| format!("{} ", cell.to_string()));
    println!("{}", display_grid);
}
