use duke::color::{Color};

pub struct Random {
  color: Color
}

impl Random {
  pub fn new(color: Color) -> Random {
    Random {
      color: color
    }
  }

  pub fn color(&self) -> Color {
    self.color
  }

}

impl super::Player for Random {
  fn turn(&self) {
    println!("Random player taking turn")
  }
}

impl super::ai::AIPlayer for Random {

}

#[cfg(test)]
mod tests {
  use super::{Random};

  use duke::board::{Board};
  use duke::color::{Color};
  use duke::player::{Player};

  #[test]
  fn color() {
    // Returns the color assigned to it.
    assert_eq!(Color::Pink, Random::new(Color::Pink).color());
  }
}
