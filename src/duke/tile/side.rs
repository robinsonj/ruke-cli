#[derive(Debug, Deserialize)]
pub struct Side {
  x:      u8,
  y:      u8
}

impl Side {
  pub fn new() -> Side {
    Side {
      x: 2,
      y: 2
    }
  }
}
