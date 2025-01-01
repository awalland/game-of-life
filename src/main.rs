use crate::CellState::{Alive, Dead};
use rand::Rng;
use std::process::Command;
use std::thread;
use std::time::Duration;

/**
* A live cell dies if it has fewer than two live neighbors.
* A live cell with two or three live neighbors lives on to the next generation.
* A live cell with more than three live neighbors dies.
* A dead cell will be brought back to live if it has exactly three live neighbors.
*/

const GRID_SIZE: usize = 130;
const SLEEP_MILLIS: u64 = 60;

#[derive(PartialEq)]
enum CellState {
    Dead,
    Alive,
}

impl CellState {
    fn character(&self) -> &str {
        match self {
            Dead => "░░░",
            Alive => "███",
        }
    }
}

struct Grid<'a> {
    cells: Vec<Vec<&'a CellState>>,
}

impl<'a> Grid<'a> {
    fn new(size: usize) -> Self {
        Grid {
            cells: vec![vec![&Dead; size]; size],
        }
    }
    fn paint(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                print!("{}", cell.character());
            }
            println!();
        }
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_grid = Grid::new(self.cells.len());

        for cell_row in 0..self.cells.len() {
            let row = &self.cells[cell_row];
            for cell_column in 0..row.len() {
                new_grid.cells[cell_row][cell_column] =
                    if rng.gen_bool(0.5) { &Alive } else { &Dead };
            }
        }
        self.cells = new_grid.cells;
    }

    fn advance(&mut self) {
        let mut new_grid = Grid::new(self.cells.len());
        for cell_row in 0..self.cells.len() {
            let row = &self.cells[cell_row];
            for cell_column in 0..row.len() {
                let current_state = row[cell_column];
                let alive_neighbors = self.active_neighbors(cell_row, cell_column, row);
                let new_state = match current_state {
                    Alive => {
                        if alive_neighbors < 2 || alive_neighbors > 3 {
                            &Dead
                        } else {
                            &Alive
                        }
                    }
                    Dead => {
                        if alive_neighbors == 3 {
                            &Alive
                        } else {
                            &Dead
                        }
                    }
                };
                new_grid.cells[cell_row][cell_column] = &new_state;
            }
        }
        self.cells = new_grid.cells
    }

    fn active_neighbors(&self, cell_row: usize, cell_column: usize, row: &Vec<&CellState>) -> i32 {
        let mut alive_neighbors = 0;
        let is_first_column = cell_column == 0;
        let is_last_column = cell_column == row.len() - 1;
        // horizontal neighbors
        // same row, left column
        if is_first_column {
            if *row.last().unwrap() == &Alive {
                alive_neighbors += 1;
            }
        } else {
            if row[cell_column - 1] == &Alive {
                alive_neighbors += 1;
            }
        }
        // same row, right column
        if is_last_column {
            if *row.first().unwrap() == &Alive {
                alive_neighbors += 1;
            }
        } else {
            if row[cell_column + 1] == &Alive {
                alive_neighbors += 1;
            }
        }

        // vertical neighbors
        let previous_row = if cell_row > 0 {
            &self.cells[cell_row - 1]
        } else {
            &self.cells.last().unwrap()
        };
        // previous row, same column
        if previous_row[cell_column] == &Alive {
            alive_neighbors += 1;
        }

        // previous row, left column
        if is_first_column {
            if *previous_row.last().unwrap() == &Alive {
                alive_neighbors += 1;
            }
        } else {
            if previous_row[cell_column - 1] == &Alive {
                alive_neighbors += 1;
            }
        }

        // previous row, right column
        if is_last_column {
            if *previous_row.first().unwrap() == &Alive {
                alive_neighbors += 1;
            }
        } else {
            if previous_row[cell_column + 1] == &Alive {
                alive_neighbors += 1;
            }
        }

        //

        let next_row = if cell_row < (self.cells.len() - 1) {
            &self.cells[cell_row + 1]
        } else {
            &self.cells.first().unwrap()
        };

        // next row, same column
        if next_row[cell_column] == &Alive {
            alive_neighbors += 1;
        }
        // next row, left column
        if is_first_column {
            if *next_row.last().unwrap() == &Alive {
                alive_neighbors += 1;
            }
        } else {
            if next_row[cell_column - 1] == &Alive {
                alive_neighbors += 1;
            }
        }
        // next row, right column
        if is_last_column {
            if *next_row.first().unwrap() == &Alive {
                alive_neighbors += 1;
            }
        } else {
            if next_row[cell_column + 1] == &Alive {
                alive_neighbors += 1;
            }
        }

        alive_neighbors
    }
}

fn main() {
    let mut grid = Grid::new(GRID_SIZE);
    grid.randomize();

    loop {
        clear_screen();
        grid.paint();
        grid.advance();
        //delay between each generation in milliseconds
        thread::sleep(Duration::from_millis(SLEEP_MILLIS));
    }
}

fn clear_screen() {
    if cfg!(unix) {
        let _ = Command::new("clear").status();
    } else if cfg!(windows) {
        let _ = Command::new("cls").status();
    }
}
