#[macro_use]
extern crate serde_derive;

mod duke;

fn main() {
  duke::Game::new().run();
}
