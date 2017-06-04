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
  pub fn new(name: String, a: Side, b: Side) -> Tile {
    Tile {
      name: name,
      a:    a,
      b:    b
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Tile};
  use duke::tile::side::Side;

  #[test]
  fn initialize() {
    let a     = Side::new(2, 2);
    let b     = Side::new(2, 2);
    let tile  = Tile::new("Test".to_string(), a, b);

    assert_eq!(tile.name, "Test".to_string());
    assert_eq!(tile.a.x(), 2);
    assert_eq!(tile.a.y(), 2);
    assert_eq!(tile.b.x(), 2);
    assert_eq!(tile.b.y(), 2);
  }
}
