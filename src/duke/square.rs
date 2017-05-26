use std::fmt;

use duke::rank::{Rank};
use duke::file::{File};

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub struct Square(u8);

pub const NUM_SQUARES: usize = 36;

impl Square {
  /// Create a new square from the given index.
  pub fn new(i: u8) -> Square {
    Square(i)
  }

  pub fn make(r: Rank, f: File) -> Square {
    Square(6 * (r.index() as u8) + (f.index() as u8))
  }

  /// Get the square's rank.
  pub fn rank(&self) -> Rank {
    Rank::from_index((self.0 / 6) as usize)
  }

  /// Get the square's file.
  pub fn file(&self) -> File {
    File::from_index((self.0 % 6) as usize)
  }

  /// Get the square one rank up.
  pub fn up(&self) -> Option<Square> {
    if self.rank() == Rank::Sixth {
      None
    } else {
      Some(Square::make(self.rank().up(), self.file()))
    }
  }

  /// Get the square one rank down.
  pub fn down(&self) -> Option<Square> {
    if self.rank() == Rank::First {
      None
    } else {
      Some(Square::make(self.rank().down(), self.file()))
    }
  }

  /// Get the square one file right.
  pub fn right(&self) -> Option<Square> {
    if self.file() == File::F {
      None
    } else {
      Some(Square::make(self.rank(), self.file().right()))
    }
  }

  /// Get the square one file left.
  pub fn left(&self) -> Option<Square> {
    if self.file() == File::A {
      None
    } else {
      Some(Square::make(self.rank(), self.file().left()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Square, NUM_SQUARES};
  use duke::rank::{Rank};
  use duke::file::{File};

  #[test]
  fn num_squares() {
    assert_eq!(36, NUM_SQUARES);
  }

  #[test]
  fn make() {
    assert_eq!(Square::new( 0), Square::make(Rank::First,   File::A));
    assert_eq!(Square::new( 1), Square::make(Rank::First,   File::B));
    assert_eq!(Square::new( 2), Square::make(Rank::First,   File::C));
    assert_eq!(Square::new( 3), Square::make(Rank::First,   File::D));
    assert_eq!(Square::new( 4), Square::make(Rank::First,   File::E));
    assert_eq!(Square::new( 5), Square::make(Rank::First,   File::F));

    assert_eq!(Square::new( 6), Square::make(Rank::Second,  File::A));
    assert_eq!(Square::new( 7), Square::make(Rank::Second,  File::B));
    assert_eq!(Square::new( 8), Square::make(Rank::Second,  File::C));
    assert_eq!(Square::new( 9), Square::make(Rank::Second,  File::D));
    assert_eq!(Square::new(10), Square::make(Rank::Second,  File::E));
    assert_eq!(Square::new(11), Square::make(Rank::Second,  File::F));

    assert_eq!(Square::new(12), Square::make(Rank::Third,   File::A));
    assert_eq!(Square::new(13), Square::make(Rank::Third,   File::B));
    assert_eq!(Square::new(14), Square::make(Rank::Third,   File::C));
    assert_eq!(Square::new(15), Square::make(Rank::Third,   File::D));
    assert_eq!(Square::new(16), Square::make(Rank::Third,   File::E));
    assert_eq!(Square::new(17), Square::make(Rank::Third,   File::F));

    assert_eq!(Square::new(18), Square::make(Rank::Fourth,  File::A));
    assert_eq!(Square::new(19), Square::make(Rank::Fourth,  File::B));
    assert_eq!(Square::new(20), Square::make(Rank::Fourth,  File::C));
    assert_eq!(Square::new(21), Square::make(Rank::Fourth,  File::D));
    assert_eq!(Square::new(22), Square::make(Rank::Fourth,  File::E));
    assert_eq!(Square::new(23), Square::make(Rank::Fourth,  File::F));

    assert_eq!(Square::new(24), Square::make(Rank::Fifth,   File::A));
    assert_eq!(Square::new(25), Square::make(Rank::Fifth,   File::B));
    assert_eq!(Square::new(26), Square::make(Rank::Fifth,   File::C));
    assert_eq!(Square::new(27), Square::make(Rank::Fifth,   File::D));
    assert_eq!(Square::new(28), Square::make(Rank::Fifth,   File::E));
    assert_eq!(Square::new(29), Square::make(Rank::Fifth,   File::F));

    assert_eq!(Square::new(30), Square::make(Rank::Sixth,   File::A));
    assert_eq!(Square::new(31), Square::make(Rank::Sixth,   File::B));
    assert_eq!(Square::new(32), Square::make(Rank::Sixth,   File::C));
    assert_eq!(Square::new(33), Square::make(Rank::Sixth,   File::D));
    assert_eq!(Square::new(34), Square::make(Rank::Sixth,   File::E));
    assert_eq!(Square::new(35), Square::make(Rank::Sixth,   File::F));
  }

  #[test]
  fn rank() {
    assert_eq!(Rank::First,   Square::new(0).rank());
    assert_eq!(Rank::Second,  Square::new(6).rank());
    assert_eq!(Rank::Third,   Square::new(12).rank());
    assert_eq!(Rank::Fourth,  Square::new(18).rank());
    assert_eq!(Rank::Fifth,   Square::new(24).rank());
    assert_eq!(Rank::Sixth,   Square::new(30).rank());
  }

  #[test]
  fn file() {
    assert_eq!(File::A, Square::new(0).file());
    assert_eq!(File::B, Square::new(7).file());
    assert_eq!(File::C, Square::new(14).file());
    assert_eq!(File::D, Square::new(21).file());
    assert_eq!(File::E, Square::new(28).file());
    assert_eq!(File::F, Square::new(35).file());
  }

  #[test]
  fn up() {
    assert_eq!(Some(Square::new( 6)), Square::new(0).up());
    assert_eq!(Some(Square::new(12)), Square::new(6).up());
    assert_eq!(Some(Square::new(18)), Square::new(12).up());
    assert_eq!(Some(Square::new(24)), Square::new(18).up());
    assert_eq!(Some(Square::new(30)), Square::new(24).up());
    assert_eq!(None,                  Square::new(30).up());
  }

  #[test]
  fn down() {
    assert_eq!(None,                  Square::new(0).down());
    assert_eq!(Some(Square::new( 0)), Square::new(6).down());
    assert_eq!(Some(Square::new( 6)), Square::new(12).down());
    assert_eq!(Some(Square::new(12)), Square::new(18).down());
    assert_eq!(Some(Square::new(18)), Square::new(24).down());
    assert_eq!(Some(Square::new(24)), Square::new(30).down());
  }

  #[test]
  fn right() {
    assert_eq!(Some(Square::new(1)),  Square::new(0).right());
    assert_eq!(Some(Square::new(2)),  Square::new(1).right());
    assert_eq!(Some(Square::new(3)),  Square::new(2).right());
    assert_eq!(Some(Square::new(4)),  Square::new(3).right());
    assert_eq!(Some(Square::new(5)),  Square::new(4).right());
    assert_eq!(None,                  Square::new(5).right());
  }

  #[test]
  fn left() {
    assert_eq!(None,                  Square::new(0).left());
    assert_eq!(Some(Square::new(0)),  Square::new(1).left());
    assert_eq!(Some(Square::new(1)),  Square::new(2).left());
    assert_eq!(Some(Square::new(2)),  Square::new(3).left());
    assert_eq!(Some(Square::new(3)),  Square::new(4).left());
    assert_eq!(Some(Square::new(4)),  Square::new(5).left());
  }
}
