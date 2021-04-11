use rust_game_of_life::cell;

fn main() {
    let mycell = cell::Cell::new(cell::CellState::ALIVE, 0, 0);

    println!("{:?}", mycell);
}
