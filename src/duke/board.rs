use std::fmt;

use super::space::*;

const NUM_SPACES: usize = 100;

pub struct Board {
  spaces: Vec<Space>
}

///
/// X X X X X X X X X X 00..09
/// X X X X X X X X X X 10..19
/// X X _ _ s s _ _ X X 20..29
/// X X _ _ _ _ _ _ X X 30..39
/// X X _ _ _ _ _ _ X X 40..49
/// X X _ _ _ _ _ _ X X 50..59
/// X X _ _ _ _ _ _ X X 60..69
/// X X _ _ s s _ _ X X 70..79
/// X X X X X X X X X X 80..89
/// X X X X X X X X X X 90..99
///
///
impl Board {
  pub fn new() -> Board {
    Board {
      spaces: Vec::with_capacity(NUM_SPACES)
    }
  }

  pub fn init(&mut self) {
    for i in 0..NUM_SPACES {
      let space_type = match i {
        24 | 25   => SpaceType::BlueStart,
        74 | 75   => SpaceType::PinkStart,
        22 ... 27 => SpaceType::Playable,
        32 ... 37 => SpaceType::Playable,
        42 ... 47 => SpaceType::Playable,
        52 ... 57 => SpaceType::Playable,
        62 ... 67 => SpaceType::Playable,
        72 ... 77 => SpaceType::Playable,
        _ => SpaceType::OutOfBounds
      };

      self.spaces.push(Space::new(space_type));
    }
  }
}

impl fmt::Debug for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut board = String::new();

    if self.spaces.len() == 0 { return write!(f, "Uninitialized."); }

    for x in 0..10 {
      board.push_str("\n   ");
      for y in 0..10 {
        board.push_str(&format!("{}", self.spaces[(y * 10) + x]));
        if y != 10 {
          board.push_str(" ");
        }
      }
    }

    write!(f, "{}", board)
  }
}
