pub mod config;
pub mod side;

use duke::tile::side::{Side};

#[derive(Debug, Deserialize)]
pub struct Tile {
  name: String,
  a:    Side,
  b:    Side
}

impl Tile {
  pub fn new(name: String) -> Tile {
    Tile {
      name: name,
      a:    Side::new(),
      b:    Side::new()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Tile};

  #[test]
  fn initialize() {
    let tile = Tile::new("Test".to_string());

    assert_eq!(tile.name, "Test".to_string());
  }
}
