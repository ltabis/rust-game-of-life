use rust_game_of_life::universe::{ Universe, Graphics };

fn main() {
    let mut my_graphics = Graphics::default();
    let mut my_universe = Universe::default();

    my_universe.add_cell(14, 14);
    my_universe.add_cell(14, 15);
    my_universe.add_cell(14, 16);

    my_universe.run(&mut my_graphics);
}