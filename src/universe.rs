// universe.rs
// holds the definition of the universe,
// where all cells will develop.
//
// by ltabis

extern crate piston_window;

use std::{ vec::Vec };
use piston_window::*;

use super::cell::{ Cell, CellState, CELL_SIZE };

const DEFAULT_PLAYGROUND_X_SIZE: u32 = 20;
const DEFAULT_PLAYGROUND_Y_SIZE: u32 = 20;

const DEFAULT_WINDOW_X_SIZE: u32 = 500;
const DEFAULT_WINDOW_Y_SIZE: u32 = 500;

pub struct Universe {
    pub playground: Vec<Cell>,
    x: u32,
    y: u32,
    display_x: u32,
    display_y: u32,
}

impl Universe {

    fn draw_playground(&self, grph: &mut Graphics, event: &Event) {
        grph.window.draw_2d(event, |context, graphics, _device | {
            // clearing the screen from last frame.
            clear([0.2; 4], graphics);

            // drawing each cells individually.
            // ! this is slow.
            // TODO: refactore this code. Idea: only draw ALIVE cells.
            for cell in self.playground.iter() {

                // println!("{}, {}, {}, {}",
                //     (self.display_x + (cell.x * CELL_SIZE)) as f64,
                //     (self.display_y + (cell.y * CELL_SIZE)) as f64,
                //     (self.display_x + ((cell.x + 1) * CELL_SIZE)) as f64,
                //     (self.display_y + ((cell.y + 1) * CELL_SIZE)) as f64
                // );

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
            // panic!();
        });
    }

    pub fn default() -> Self {
        let mut playground = Vec::new();

        // ! this is slow.
        // TODO: refactore this code. Idea: only generate ALIVE cells.
        for x in 0..DEFAULT_PLAYGROUND_X_SIZE {
            for y in 0..DEFAULT_PLAYGROUND_Y_SIZE {
                playground.push(Cell::new(CellState::DEAD, x, y));
            }
        }

        Universe {
            playground,
            x: DEFAULT_PLAYGROUND_X_SIZE,
            y: DEFAULT_PLAYGROUND_Y_SIZE,
            display_x: (DEFAULT_WINDOW_X_SIZE / 2) - (DEFAULT_PLAYGROUND_X_SIZE / 2 * CELL_SIZE),
            display_y: (DEFAULT_WINDOW_Y_SIZE / 2) - (DEFAULT_PLAYGROUND_Y_SIZE / 2 * CELL_SIZE),
        }
    }

    pub fn new(x_size: u32, y_size: u32) -> Self {
        let mut playground = Vec::new();

        // ! this is slow.
        for x in 0..x_size {
            for y in 0..y_size {
                playground.push(Cell::new(CellState::DEAD, x, y));
            }
        }

        Universe {
            playground,
            x: x_size,
            y: y_size,
            display_x: (DEFAULT_WINDOW_X_SIZE / 2) - (x_size / 2 * CELL_SIZE),
            display_y: (DEFAULT_WINDOW_Y_SIZE / 2) - (y_size / 2 * CELL_SIZE),
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
