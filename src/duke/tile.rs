pub struct Tile {
  name: String
}

impl Tile {
  pub fn new(name: String) -> Tile {
    Tile {
      name: name
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Tile};

  #[test]
  fn initialize() {
    let tile = Tile::new("Test".to_string());
    assert_eq!("Test".to_string(), tile.name);
  }
}
