mod utils;

use wasm_bindgen::prelude::*;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;


extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet
}

#[wasm_bindgen]
pub enum Pattern {
    Glider,
    Pentomino
}


impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        let n = self.get_index(north, column);
        let ne = self.get_index(north, east);
        let w = self.get_index(row, west);
        let e = self.get_index(row, east);
        let sw = self.get_index(south, west);
        let s = self.get_index(south, column);
        let se = self.get_index(south, east);
        return self.cells[nw] as u8 + self.cells[ne] as u8 + self.cells[n] as u8
            + self.cells[w] as u8 + self.cells[e] as u8
            + self.cells[s] as u8 + self.cells[sw] as u8 + self.cells[se] as u8;
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                });
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn set_preset(&mut self, preset: u8) {
        let size = (self.width * self.height) as usize;
        let mut new_cells = FixedBitSet::with_capacity(size);
        match preset {
            0 => {
                for i in 0..size {
                    new_cells.set(i, i % 3 == 0 || i % 8 == 0 || i % 7 == 0);
                }
            },
            _ => ()
        }
        self.cells = new_cells;
    }

    /// Set the width of the universe.
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size)
    }

    /// Set the height of the universe.
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * height) as usize;
        self.cells = FixedBitSet::with_capacity(size)
    }

    pub fn toggle_cell(&mut self, row: u32, col:u32) {
        self.cells.toggle(self.get_index(row, col))
    }

    pub fn add_pattern(&mut self, row: u32, col:u32, pattern: Pattern) {
        let deltas  = match pattern {
            Pattern::Glider => [(0, self.width - 1), (1, 0), (1, 1), (0, 1), (self.height - 1, 1)],
            Pattern::Pentomino => [(0, self.width - 1), (0, 0), (1, 0), (self.height - 1, 0), (self.height - 1, 1)]
        };
        for (delta_row, delta_col) in deltas.iter() {
            let neighbor_row = (row + delta_row) % self.height;
            let neighbor_col = (col + delta_col) % self.width;
            self.cells.set(self.get_index(neighbor_row, neighbor_col), true);
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

}


use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
