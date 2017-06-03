extern crate rand;

#[macro_use]
extern crate serde_derive;

mod bitboard;
mod duke;

fn main() {
  duke::Game::new().run();
}
