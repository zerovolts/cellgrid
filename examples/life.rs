use rand;

use autocell::grid::{Coord, Grid};

fn main() {
    let mut life_board = LifeBoard::random((16, 16));

    for _i in 0..10 {
        life_board.print();
        println!();
        life_board.step();
    }
}

struct LifeBoard {
    grid: Grid<LifeState>,
}

impl LifeBoard {
    fn random(dimensions: (u32, u32)) -> Self {
        Self {
            grid: Grid::with_generator(dimensions, (0, 0), |_coord| match rand::random::<bool>() {
                true => LifeState::Alive,
                false => LifeState::Dead,
            }),
        }
    }

    fn step(&mut self) {
        let neighbor_counts = self
            .grid
            .iter()
            .map(|cell| self.live_neighbor_count(cell.0))
            .collect::<Vec<_>>();

        for (cell, neighbor_count) in self.grid.cell_iter_mut().zip(neighbor_counts) {
            *cell = LifeBoard::compute_state(*cell, neighbor_count)
        }
    }

    fn live_neighbor_count(&self, coord: Coord) -> usize {
        let neighbor_coords = coord.neighbor_coords().collect::<Vec<_>>();
        self.grid
            .selection_iter(&neighbor_coords)
            .filter(|cell| *cell.1 == LifeState::Alive)
            .count()
    }

    fn compute_state(state: LifeState, neighbor_count: usize) -> LifeState {
        match state {
            LifeState::Alive => {
                if neighbor_count == 2 || neighbor_count == 3 {
                    LifeState::Alive
                } else {
                    LifeState::Dead
                }
            }
            LifeState::Dead => {
                if neighbor_count == 3 {
                    LifeState::Alive
                } else {
                    LifeState::Dead
                }
            }
        }
    }

    // TODO: Clean this up and make it into a trait for Grid.
    fn print(&self) {
        for y in 0..(self.grid.dimensions.1 as i32) {
            for x in 0..(self.grid.dimensions.0 as i32) {
                let coord = Coord(x, y);
                print!(
                    "{} ",
                    match self.grid.get(coord) {
                        Some(LifeState::Alive) => 'O',
                        Some(LifeState::Dead) => '∙',
                        None => '?',
                    }
                );
            }
            println!();
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LifeState {
    Dead,
    Alive,
}

impl Default for LifeState {
    fn default() -> Self {
        LifeState::Dead
    }
}
