// universe.rs
// holds the definition of the universe,
// where all cells will develop.
//
// by ltabis

extern crate piston_window;

use std::{ vec::Vec };
use piston_window::*;

use super::cell::{ Cell, CellState };

const DEFAULT_PLAYGROUND_X_SIZE: u32 = 10;
const DEFAULT_PLAYGROUND_Y_SIZE: u32 = 10;

const DEFAULT_WINDOW_X_SIZE: u32 = 1920;
const DEFAULT_WINDOW_Y_SIZE: u32 = 1080;

pub struct Universe {
    pub playground: Vec<Cell>,
}

impl Universe {

    fn draw_playground(&self, grph: &mut Graphics, event: &Event) {
        grph.window.draw_2d(event, |context, graphics, _device | {
            // clearing the screen from last frame.
            clear([1.0; 4], graphics);

            // drawing each cells individually.
            // ! this is slow.
            // TODO: refactore this code. Idea: only draw ALIVE cells.
            for cell in self.playground.iter() {
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [0.0, 0.0, 100.0, 100.0],
                    context.transform,
                    graphics
                );
            }
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

        Universe { playground }
    }

    pub fn new(x_size: u32, y_size: u32) -> Self {
        let mut playground = Vec::new();

        // ! this is slow.
        for x in 0..x_size {
            for y in 0..y_size {
                playground.push(Cell::new(CellState::DEAD, x, y));
            }
        }

        Universe { playground }
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
