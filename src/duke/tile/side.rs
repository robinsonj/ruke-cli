#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub struct Side {
  x:      u8,
  y:      u8
}

impl Side {
  pub fn new(x: u8, y: u8) -> Side {
    Side {
      x: x,
      y: y
    }
  }

  pub fn x(&self) -> u8 {
    self.x
  }

  pub fn y(&self) -> u8 {
    self.y
  }
}

#[cfg(test)]
mod tests {
  use super::{Side};

  #[test]
  fn new() {
    assert_eq!(Side { x: 2, y: 2}, Side::new(2, 2));
  }
}
