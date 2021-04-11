// cell.rs
// olds the definition of cells.
//
// by ltabis

// describe the state of a cell.
#[derive(Debug)]
pub enum CellState {
    ALIVE,
    DEAD
}

// olds data for one cell.
#[derive(Debug)]
pub struct Cell {
    state: CellState,
    x: i32,
    y: i32
}

impl Cell {
    pub fn new(state: CellState, x: i32, y: i32) -> Self {
        Cell {
            state,
            x,
            y
        }
    }
}