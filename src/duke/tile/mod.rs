pub mod config;
pub mod side;

use duke::tile::side::{Side};

#[derive(Debug, Deserialize)]
pub struct Tile {
  name: String,
  A:    Side,
  B:    Side
}

impl Tile {
  pub fn new(name: String) -> Tile {
    Tile {
      name: name,
      A:    Side::new(),
      B:    Side::new()
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
