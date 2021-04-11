// cell.rs
// holds the definition of cells.
//
// by ltabis

// describe the state of a cell.
#[derive(Debug)]
pub enum CellState {
    ALIVE,
    DEAD
}

// holds data for one cell.
#[derive(Debug)]
pub struct Cell {
    state: CellState,
    x: u32,
    y: u32
}

impl Cell {
    pub fn new(state: CellState, x: u32, y: u32) -> Self {
        Cell {
            state,
            x,
            y
        }
    }
}