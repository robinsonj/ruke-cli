pub mod config;

///
///
/// Tile.x : "Starting square" horizontal position the tile describes.
/// Tile.y : "Starting square" vertical position the tile describes.
///
#[derive(Debug)]
pub struct Tile {
  name: String,
  x:    u8,
  y:    u8
}

impl Tile {
  pub fn new(name: String, x: u8, y: u8) -> Tile {
    Tile {
      name: name,
      x:    x,
      y:    y
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Tile};

  #[test]
  fn initialize() {
    let tile = Tile::new("Test".to_string(), 2, 2);

    assert_eq!(tile.name, "Test".to_string());
    assert_eq!(tile.x, 2);
    assert_eq!(tile.y, 2);
  }
}
