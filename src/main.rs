use rust_game_of_life::universe::{ Universe, Graphics };

fn main() {
    let mut my_graphics = Graphics::default();
    let mut my_universe = Universe::default();

    my_universe.run(&mut my_graphics);
}