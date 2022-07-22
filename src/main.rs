
//mod game;
mod board;
mod piece;
mod square;

fn main() {
    let board = board::Board::new();
    println!("{}", board);
}

