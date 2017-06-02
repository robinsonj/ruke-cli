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

  /// Get the color's backrank.
  pub fn backrank(&self) -> Rank {
    match *self {
      Color::Pink => Rank::First,
      Color::Blue => Rank::Sixth
    }
  }

  /// Get the opponent colors' backrank.
  pub fn opponent_backrank(&self) -> Rank {
    (!*self).backrank()
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
  use super::{Color, ALL_COLORS, NUM_COLORS};
  use duke::rank::Rank;

  #[test]
  fn num_colors() {
    assert_eq!(2, NUM_COLORS);
  }

  #[test]
  fn all_colors() {
    assert_eq!(2, ALL_COLORS.len());
    assert_eq!(ALL_COLORS, [
      Color::Pink,
      Color::Blue
    ]);
  }

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
  fn opponent_backrank() {
    assert_eq!(Rank::Sixth, Color::Pink.opponent_backrank());
    assert_eq!(Rank::First, Color::Blue.opponent_backrank());
  }

  #[test]
  fn not() {
    assert_eq!(!Color::Pink, Color::Blue);
    assert_eq!(!Color::Blue, Color::Pink);
  }
}
