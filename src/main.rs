use crate::CellState::{Alive, Dead, Infected};
use rand::Rng;
use std::thread;
use std::time::Duration;

/**
* A live cell dies if it has fewer than two live neighbors.
* A live cell with two or three live neighbors lives on to the next generation.
* A live cell with more than three live neighbors dies.
* A dead cell will be brought back to live if it has exactly three live neighbors.
*/

#[derive(PartialEq)]
enum CellState {
    Dead,
    Alive,
    Infected,
}

impl CellState {
    fn character(&self) -> &str {
        match self {
            Dead => "░░░",
            Alive => "███",
            Infected => "XXX",
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
        println!();
        println!();
        println!();
        println!();

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
                let num = rng.gen_range(0..100);
                new_grid.cells[cell_row][cell_column] = if num <= 51 {
                    &Alive
                } else if num <= 98 {
                    &Dead
                } else {
                    &Infected
                };
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
                let infected_neighbors = self.infected_neighbors(cell_row, cell_column, row);
                let new_state = match current_state {
                    Alive => {
                        if infected_neighbors > 1 {
                            &Infected
                        } else if alive_neighbors < 2 || alive_neighbors > 3 {
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
                    Infected => &Infected,
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
        if !is_first_column {
            // same row, left column
            if row[cell_column - 1] == &Alive {
                alive_neighbors += 1;
            }
        }

        if !is_last_column {
            // same row, right column
            if row[cell_column + 1] == &Alive {
                alive_neighbors += 1;
            }
        }

        if cell_row > 0 {
            let previous_row = &self.cells[cell_row - 1];
            // previous row, same column
            if previous_row[cell_column] == &Alive {
                alive_neighbors += 1;
            }
            // previous row, left column
            if !is_first_column && previous_row[cell_column - 1] == &Alive {
                alive_neighbors += 1;
            }
            // previous row, right column
            if !is_last_column && previous_row[cell_column + 1] == &Alive {
                alive_neighbors += 1;
            }
        }
        if cell_row < self.cells.len() - 1 {
            let next_row = &self.cells[cell_row + 1];
            // next row, same column
            if next_row[cell_column] == &Alive {
                alive_neighbors += 1;
            }
            // next row, left column
            if !is_first_column && next_row[cell_column - 1] == &Alive {
                alive_neighbors += 1;
            }
            // next row, right column
            if !is_last_column && next_row[cell_column + 1] == &Alive {
                alive_neighbors += 1;
            }
        }
        alive_neighbors
    }
    fn infected_neighbors(
        &self,
        cell_row: usize,
        cell_column: usize,
        row: &Vec<&CellState>,
    ) -> i32 {
        let mut alive_neighbors = 0;
        let is_first_column = cell_column == 0;
        let is_last_column = cell_column == row.len() - 1;
        if !is_first_column {
            // same row, left column
            if row[cell_column - 1] == &Infected {
                alive_neighbors += 1;
            }
        }

        if !is_last_column {
            // same row, right column
            if row[cell_column + 1] == &Infected {
                alive_neighbors += 1;
            }
        }

        if cell_row > 0 {
            let previous_row = &self.cells[cell_row - 1];
            // previous row, same column
            if previous_row[cell_column] == &Infected {
                alive_neighbors += 1;
            }
            // previous row, left column
            if !is_first_column && previous_row[cell_column - 1] == &Infected {
                alive_neighbors += 1;
            }
            // previous row, right column
            if !is_last_column && previous_row[cell_column + 1] == &Infected {
                alive_neighbors += 1;
            }
        }
        if cell_row < self.cells.len() - 1 {
            let next_row = &self.cells[cell_row + 1];
            // next row, same column
            if next_row[cell_column] == &Infected {
                alive_neighbors += 1;
            }
            // next row, left column
            if !is_first_column && next_row[cell_column - 1] == &Infected {
                alive_neighbors += 1;
            }
            // next row, right column
            if !is_last_column && next_row[cell_column + 1] == &Infected {
                alive_neighbors += 1;
            }
        }
        alive_neighbors
    }
}

fn main() {
    const GRID_SIZE: usize = 30;

    let mut grid = Grid::new(GRID_SIZE);
    grid.randomize();

    loop {
        clear_screen();
        grid.paint();
        grid.advance();
        //delay between each generation in milliseconds
        thread::sleep(Duration::from_millis(60));
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
