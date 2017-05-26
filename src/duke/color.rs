use std::ops::Not;

use duke::rank::Rank;

/// Represent the player colors.
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum Color {
  Pink,
  Blue
}

/// Number of player colors.
pub const NUM_COLORS: usize = 2;

/// Color list.
pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::Pink, Color::Blue];

impl Color {
  /// Convert the color to a usize.
  pub fn index(&self) -> usize {
    *self as usize
  }

  /// Convert a given color to a rank.
  pub fn backrank(&self) -> Rank {
    match *self {
      Color::Pink => Rank::First,
      Color::Blue => Rank::Sixth
    }
  }
}

impl Not for Color {
  type Output = Color;

  /// Get the opponent color.
  fn not(self) -> Color {
    if self == Color::Blue {
      Color::Pink
    } else {
      Color::Blue
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Color;
  use duke::rank::Rank;

  #[test]
  fn index() {
    let pink = Color::Pink;
    let blue = Color::Blue;

    assert_eq!(0, pink.index());
    assert_eq!(1, blue.index());
  }

  #[test]
  fn backrank() {
    assert_eq!(Rank::First, Color::Pink.backrank());
    assert_eq!(Rank::Sixth, Color::Blue.backrank());
  }

  #[test]
  fn not() {
    assert_eq!(!Color::Pink, Color::Blue);
    assert_eq!(!Color::Blue, Color::Pink);
  }
}