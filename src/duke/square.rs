use std::fmt;

use duke::color::{Color};
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

  /// Create a new square given a Rake and File.
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

  /// Given a player color, get the Square one square forwards.
  pub fn player_forward(&self, color: Color) -> Option<Square> {
    match color {
      Color::Pink => self.up(),
      Color::Blue => self.down()
    }
  }

  /// Given a player color, get the Square one square backwards.
  pub fn player_backward(&self, color: Color) -> Option<Square> {
    match color {
      Color::Pink => self.down(),
      Color::Blue => self.up()
    }
  }

  /// Given a player color, get the Square one square left.
  pub fn player_left(&self, color: Color) -> Option<Square> {
    match color {
      Color::Pink => self.left(),
      Color::Blue => self.right()
    }
  }

  /// Given a player color, get the Square one square right.
  pub fn player_right(&self, color: Color) -> Option<Square> {
    match color {
      Color::Pink => self.right(),
      Color::Blue => self.left()
    }
  }
}

impl fmt::Display for Square {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", (('a' as u8) + ((self.0 % 6) as u8)) as char,
                      (('1' as u8) + ((self.0 / 6) as u8)) as char
    )
  }
}

#[cfg(test)]
mod tests {
  use super::{Square, NUM_SQUARES};
  use duke::color::{Color};
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

  #[test]
  fn player_forward() {
    assert_eq!(Some(Square::new(6)),  Square::new(0).player_forward(Color::Pink));
    assert_eq!(None,                  Square::new(30).player_forward(Color::Pink));

    assert_eq!(None,                  Square::new(0).player_forward(Color::Blue));
    assert_eq!(Some(Square::new(24)), Square::new(30).player_forward(Color::Blue));
  }

  #[test]
  fn player_backward() {
    assert_eq!(None,                  Square::new(0).player_backward(Color::Pink));
    assert_eq!(Some(Square::new(24)), Square::new(30).player_backward(Color::Pink));

    assert_eq!(Some(Square::new(6)),  Square::new(0).player_backward(Color::Blue));
    assert_eq!(None,                  Square::new(30).player_backward(Color::Blue));
  }

  #[test]
  fn player_left() {
    assert_eq!(None,                  Square::new(0).player_left(Color::Pink));
    assert_eq!(Some(Square::new(34)), Square::new(35).player_left(Color::Pink));

    assert_eq!(Some(Square::new(1)),  Square::new(0).player_left(Color::Blue));
    assert_eq!(None,                  Square::new(35).player_left(Color::Blue));
  }

  #[test]
  fn player_right() {
    assert_eq!(Some(Square::new(1)),  Square::new(0).player_right(Color::Pink));
    assert_eq!(None,                  Square::new(35).player_right(Color::Pink));

    assert_eq!(None,                  Square::new(0).player_right(Color::Blue));
    assert_eq!(Some(Square::new(34)), Square::new(35).player_right(Color::Blue));
  }

  #[test]
  fn display() {
    assert_eq!("a1".to_owned(), format!("{}", Square::new(0)));
    assert_eq!("a2".to_owned(), format!("{}", Square::new(6)));
    assert_eq!("a3".to_owned(), format!("{}", Square::new(12)));
    assert_eq!("a4".to_owned(), format!("{}", Square::new(18)));
    assert_eq!("a5".to_owned(), format!("{}", Square::new(24)));
    assert_eq!("a6".to_owned(), format!("{}", Square::new(30)));

    assert_eq!("b1".to_owned(), format!("{}", Square::new(1)));
    assert_eq!("b2".to_owned(), format!("{}", Square::new(7)));
    assert_eq!("b3".to_owned(), format!("{}", Square::new(13)));
    assert_eq!("b4".to_owned(), format!("{}", Square::new(19)));
    assert_eq!("b5".to_owned(), format!("{}", Square::new(25)));
    assert_eq!("b6".to_owned(), format!("{}", Square::new(31)));

    assert_eq!("c1".to_owned(), format!("{}", Square::new(2)));
    assert_eq!("c2".to_owned(), format!("{}", Square::new(8)));
    assert_eq!("c3".to_owned(), format!("{}", Square::new(14)));
    assert_eq!("c4".to_owned(), format!("{}", Square::new(20)));
    assert_eq!("c5".to_owned(), format!("{}", Square::new(26)));
    assert_eq!("c6".to_owned(), format!("{}", Square::new(32)));

    assert_eq!("d1".to_owned(), format!("{}", Square::new(3)));
    assert_eq!("d2".to_owned(), format!("{}", Square::new(9)));
    assert_eq!("d3".to_owned(), format!("{}", Square::new(15)));
    assert_eq!("d4".to_owned(), format!("{}", Square::new(21)));
    assert_eq!("d5".to_owned(), format!("{}", Square::new(27)));
    assert_eq!("d6".to_owned(), format!("{}", Square::new(33)));

    assert_eq!("e1".to_owned(), format!("{}", Square::new(4)));
    assert_eq!("e2".to_owned(), format!("{}", Square::new(10)));
    assert_eq!("e3".to_owned(), format!("{}", Square::new(16)));
    assert_eq!("e4".to_owned(), format!("{}", Square::new(22)));
    assert_eq!("e5".to_owned(), format!("{}", Square::new(28)));
    assert_eq!("e6".to_owned(), format!("{}", Square::new(34)));

    assert_eq!("f1".to_owned(), format!("{}", Square::new(5)));
    assert_eq!("f2".to_owned(), format!("{}", Square::new(11)));
    assert_eq!("f3".to_owned(), format!("{}", Square::new(17)));
    assert_eq!("f4".to_owned(), format!("{}", Square::new(23)));
    assert_eq!("f5".to_owned(), format!("{}", Square::new(29)));
    assert_eq!("f6".to_owned(), format!("{}", Square::new(35)));
  }
}
