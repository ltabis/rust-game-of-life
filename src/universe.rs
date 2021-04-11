// universe.rs
// holds the definition of the universe,
// where all cells will develop.
//
// by ltabis

extern crate piston_window;

use std::{ vec::Vec };
use piston_window::*;

use super::cell::{ Cell, CellState, CELL_SIZE };

const DEFAULT_PLAYGROUND_X_SIZE: u32 = 40;
const DEFAULT_PLAYGROUND_Y_SIZE: u32 = 40;

const DEFAULT_WINDOW_X_SIZE: u32 = 500;
const DEFAULT_WINDOW_Y_SIZE: u32 = 500;

const DEFAULT_UPDATE_TIME: f64 = 0.1;

pub struct Universe {
    pub playground: Vec<Cell>,
    x: u32,
    y: u32,
    display_x: u32,
    display_y: u32,
    last_frame: f64,
    current_frame: f64,
    update_time: f64,
}

impl Universe {

    fn new_playground(size_x: u32, size_y: u32) -> Vec<Cell> {
        let mut playground = Vec::new();

        // ! this is slow.
        // TODO: refactore this code. Idea: only generate ALIVE cells.
        for y in 0..size_y {
            for x in 0..size_x {
                playground.push(Cell::new(CellState::DEAD, x, y));
            }
        }

        playground
    }

    fn draw_playground(&self, grph: &mut Graphics, event: &Event) {
        grph.window.draw_2d(event, |context, graphics, _device | {
            // clearing the screen from last frame.
            clear([0.2; 4], graphics);

            // drawing each cells individually.
            // ! this is slow.
            // TODO: refactore this code. Idea: only draw ALIVE cells.
            for cell in self.playground.iter() {

                rectangle(
                    cell.get_state_color(),
                    [
                        (self.display_x + (cell.x * CELL_SIZE)) as f64,
                        (self.display_y + (cell.y * CELL_SIZE)) as f64,
                        (self.display_x + ((cell.x + 1) * CELL_SIZE)) as f64,
                        (self.display_y + ((cell.y + 1) * CELL_SIZE)) as f64
                    ],
                    context.transform,
                    graphics
                );
            }
        });
    }

    fn update(&mut self, event: Event) {

        // updating current frame time.
        if let Some(t) = event.update_args() {
            self.current_frame += t.dt;
        }

        // computing rules every update_time seconds.
        if self.current_frame - self.last_frame >= self.update_time {
            self.last_frame = self.current_frame;

            self.compute_rules();
        }
    }

    fn compute_rules(&mut self) {
        let mut new_playground = Universe::new_playground(self.x, self.y);

        for y in 0..self.y {
            for x in 0..self.x {
                let index = (y * self.x + x) as usize;
                let alive_cell_count = self.count_alive_neighbor_cells(index);

                match alive_cell_count {
                    2 => new_playground[index].state = self.playground[index].state,
                    3 => new_playground[index].state = CellState::ALIVE,
                    _ => new_playground[index].state = CellState::DEAD,
                }
            }
        }

        self.playground = new_playground;
    }

    fn count_alive_neighbor_cells(&self, index: usize) -> u8 {

        let cell: &Cell = &self.playground[index];
        let mut alive_cell_count = 0;

        // ! not great ...
        // TODO: refactor this.
        for y in -1i8..=1 {
            for x in -1i8..=1 {

                if x == 0 && y == 0 {
                    continue;
                }

                let new_x = match x {
                    -1 => if cell.x != 0 { cell.x - 1 } else { 0 },
                    n => cell.x + n as u32
                };

                let new_y = match y {
                    -1 => if cell.y != 0 { cell.y - 1 } else { 0 },
                    n => cell.y + n as u32
                };

                if new_x > 0 && new_y > 0 && new_x < self.x && new_y < self.y {
                    let neighbor_cell = &self.playground[(new_y * self.x + new_x) as usize];

                    match neighbor_cell.state {
                        CellState::ALIVE => { alive_cell_count += 1; },
                        CellState::DEAD => (),
                    }
                }
            }
        }

        alive_cell_count
    }

    pub fn default() -> Self {
        let playground = Universe::new_playground(DEFAULT_PLAYGROUND_X_SIZE, DEFAULT_PLAYGROUND_Y_SIZE);

        Universe {
            playground,
            x: DEFAULT_PLAYGROUND_X_SIZE,
            y: DEFAULT_PLAYGROUND_Y_SIZE,
            display_x: (DEFAULT_WINDOW_X_SIZE / 2) - (DEFAULT_PLAYGROUND_X_SIZE / 2 * CELL_SIZE),
            display_y: (DEFAULT_WINDOW_Y_SIZE / 2) - (DEFAULT_PLAYGROUND_Y_SIZE / 2 * CELL_SIZE),
            last_frame: 0.0,
            current_frame: 0.0,
            update_time: DEFAULT_UPDATE_TIME,
        }
    }

    pub fn new(x_size: u32, y_size: u32) -> Self {
        let playground = Universe::new_playground(x_size, y_size);

        Universe {
            playground,
            x: x_size,
            y: y_size,
            display_x: (DEFAULT_WINDOW_X_SIZE / 2) - (x_size / 2 * CELL_SIZE),
            display_y: (DEFAULT_WINDOW_Y_SIZE / 2) - (y_size / 2 * CELL_SIZE),
            last_frame: 0.0,
            current_frame: 0.0,
            update_time: DEFAULT_UPDATE_TIME,
        }
    }

    pub fn add_cell(&mut self, x: u32, y: u32) {

        if x > DEFAULT_PLAYGROUND_X_SIZE || y > DEFAULT_PLAYGROUND_Y_SIZE {
            println!("Couldn't create a cell at x: {} & y: {}.", x, y);
            return
        }

        // mutating the cell
        self.playground
            .iter_mut()
            .nth((((y - 1) * self.x) + x - 1) as usize)
            .unwrap()
            .state = CellState::ALIVE;
    }

    pub fn run(&mut self, graphics: &mut Graphics) {
        while let Some(event) = graphics.window.next() {
            self.draw_playground(graphics, &event);
            self.update(event);
        }
    }
}

pub struct Graphics {
    window: PistonWindow,
}

impl Graphics {
    pub fn default() -> Graphics {
        Graphics {
            window: WindowSettings::new("Game of life", [DEFAULT_WINDOW_X_SIZE, DEFAULT_WINDOW_Y_SIZE])
                .exit_on_esc(true)
                .build()
                .unwrap()
        }
    }

    pub fn new(win_x_size: u32, win_y_size: u32) -> Graphics {
        Graphics {
            window: WindowSettings::new("Game of life", [win_x_size, win_y_size])
                .exit_on_esc(true)
                .build()
                .unwrap()
        }
    }
}
