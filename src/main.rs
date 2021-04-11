use rust_game_of_life::universe::{ Universe };

fn main() {
    let my_universe = Universe::new();

    for cell in my_universe.playground.iter() {
        println!("{:?}", cell);
    }
}
