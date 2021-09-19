mod game;
mod chess;

use crate::game::game::Game;


fn main() {
    let mut game = Game::new();
    game.make_move(4, 1, 4, 3);
    print!("{}", game);
}
