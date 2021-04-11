// cell.rs
// holds the definition of cells.
//
// by ltabis

pub const CELL_SIZE: u32 = 10;

// describe the state of a cell.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum CellState {
    ALIVE,
    DEAD
}

// holds data for one cell.
#[derive(Debug)]
pub struct Cell {
    pub state: CellState,
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(state: CellState, x: u32, y: u32) -> Self {
        Cell { state, x, y }
    }

    pub fn get_state_color(&self) -> [f32; 4] {
        match &self.state {
            CellState::ALIVE => [1.0, 1.0, 1.0, 1.0],
            CellState::DEAD => [0.0, 0.0, 0.0, 1.0],
        }
    }
}