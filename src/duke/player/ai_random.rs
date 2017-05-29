use rand::{thread_rng, Rng};

use duke::board::{Board};
use duke::color::{Color};
use duke::move_type::{MoveType};
use duke::move_type::summon::*;
use duke::square::{Square};

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

  fn setup_duke(&self, board: &Board) -> Option<MoveType> {
    let valid: Vec<Square> = board.starting_squares(self.color()).to_vec();

    if valid.len() > 0 {
      let square: Square = *(thread_rng().choose(&valid).take().unwrap());
      let summon = Summon::new();
      None
    } else {
      None
    }
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
