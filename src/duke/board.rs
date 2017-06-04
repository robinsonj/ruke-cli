use std::fmt;
use bitboard::{BitBoard, EMPTY, FULL};
use duke::color::{Color};
use duke::file::{NUM_FILES};
use duke::rank::*;
use duke::square::{Square};

pub const NUM_SPACES: usize = NUM_FILES * NUM_RANKS;

pub struct Board {
  state: Vec<u8>,
  color: BitBoard,
  dukes: BitBoard
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
      state: vec![0; NUM_SPACES],
      color: EMPTY,
      dukes: EMPTY
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

  pub fn starting_squares(&self, color: Color) -> Vec<Square> {
    let mut vec = Vec::new();

    if NUM_FILES % 2 == 0 {
      vec.push(Square::new((color.backrank().index() * NUM_RANKS + ((NUM_FILES / 2) - 1)) as u8));
      vec.push(Square::new((color.backrank().index() * NUM_RANKS + ((NUM_FILES / 2))) as u8));
    }

    vec
  }
}

impl fmt::Debug for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut board = String::new();

    if self.spaces.len() == 0 { return write!(f, "Uninitialized."); }

    for x in 0..10 {
      board.push_str("\n   ");
      for y in 0..10 {
        board.push_str(&format!("{:?}", self.spaces[(y * 10) + x]));
        if y != 10 {
          board.push_str(" ");
        }
      }
    }

    write!(f, "{}", board)
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut board = String::new();

    if self.spaces.len() == 0 { return write!(f, "Uninitialized."); }

    board.push_str("\n     ---------------");

    for x in 2..8 {
      board.push_str("\n     | ");

      for y in 2..8 {
        board.push_str(&format!("{} ", self.spaces[(y * 10) + x]));
      }

      board.push_str("|");
    }

    board.push_str("\n     ---------------");

    write!(f, "{}", board)
  }
}

#[cfg(test)]
mod tests {
  use super::{Board, NUM_SPACES};
  use bitboard::{BitBoard, EMPTY};
  use duke::color::{Color};
  use duke::square::{Square};

  #[test]
  fn new() {
    assert_eq!(Board::new(), Board {
      state: vec![0; NUM_SPACES],
      color: EMPTY,
      dukes: EMPTY
    });
  }

  #[test]
  fn starting_squares() {
    let b = Board::new();

    assert_eq!(vec![Square::new(2),   Square::new(3)],
               b.starting_squares(Color::Pink));
    assert_eq!(vec![Square::new(32),  Square::new(33)],
               b.starting_squares(Color::Blue));
  }
}
