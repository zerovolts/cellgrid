use tapestry::{
    patterns::{Circle, Rect},
    VecGrid,
};

fn main() {
    let mut grid: VecGrid<char> = VecGrid::new(Rect::new((19, 19)));
    let circle = Circle::new((9, 9), 7);

    // Fill grid background
    for (_coord, cell) in grid.iter_mut() {
        *cell = '∙';
    }

    // Draw circle
    for result in grid.selection_iter_mut(circle.iter()) {
        if let Ok((_coord, cell)) = result {
            *cell = '#';
        }
    }

    println!("{}", grid);
}
