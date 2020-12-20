use rand;

use autocell::{coord::Coord, grid::Grid, patterns};

fn main() {
    let mut life_board = LifeBoard::random((16, 16));

    for _i in 0..10 {
        println!("{}", life_board.grid);
        life_board.step();
    }
}

struct LifeBoard {
    grid: Grid<LifeState>,
}

impl LifeBoard {
    fn random(dimensions: (u32, u32)) -> Self {
        Self {
            grid: Grid::with_generator(dimensions, (0, 0), |(_x, _y)| {
                match rand::random::<bool>() {
                    true => LifeState::Alive,
                    false => LifeState::Dead,
                }
            }),
        }
    }

    fn step(&mut self) {
        let neighbor_counts = self
            .grid
            .iter()
            .map(|cell| self.live_neighbor_count(cell.0))
            // `collect` to release the borrow on `self`.
            .collect::<Vec<_>>();

        for (cell, neighbor_count) in self.grid.cell_iter_mut().zip(neighbor_counts) {
            *cell = LifeBoard::compute_state(*cell, neighbor_count)
        }
    }

    fn live_neighbor_count(&self, coord: Coord) -> usize {
        let neighbor_coords = patterns::neighbor_coords().map(|offset| coord + offset);
        self.grid
            .selection_iter(neighbor_coords)
            .filter(|r_cell| {
                if let Ok(cell) = r_cell {
                    *cell.1 == LifeState::Alive
                } else {
                    false
                }
            })
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

impl From<LifeState> for char {
    fn from(ls: LifeState) -> char {
        match ls {
            LifeState::Alive => 'O',
            LifeState::Dead => '∙',
        }
    }
}
