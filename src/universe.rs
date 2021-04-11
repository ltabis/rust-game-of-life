// universe.rs
// holds the definition of the universe,
// where all cells will develop.
//
// by ltabis

use std::{ vec::Vec };

use cell::{ Cell, CellState };

use super::cell;

const DEFAULT_X_SIZE: u32 = 10;
const DEFAULT_Y_SIZE: u32 = 10;

pub struct Universe {
    pub playground: Vec<cell::Cell>,
}

impl Universe {
    pub fn new() -> Self {
        let mut playground = Vec::new();

        // ! this is slow.
        for x in 0..DEFAULT_X_SIZE {
            for y in 0..DEFAULT_Y_SIZE {
                playground.push(Cell::new(CellState::DEAD, x, y));
            }
        }

        Universe { playground }
    }
}