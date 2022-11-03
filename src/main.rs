use game::Game;
use io::clear;

mod event;
mod game;
mod io;
mod location;
mod map;
mod member;
mod party;
mod tile;
mod time;

fn main() {
    clear();
    let mut game = Game::load_or_new();
    game.run();
}
