use rust_game_of_life::universe::{ Universe, Graphics };

fn main() {
    let mut my_graphics = Graphics::default();
    let mut my_universe = Universe::default();

    my_universe.add_cell(1, 1);
    my_universe.add_cell(9, 9);
    my_universe.add_cell(20, 20);

    my_universe.run(&mut my_graphics);
}