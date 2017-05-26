/// Represent a board rank.
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum Rank {
  First,
  Second,
  Third,
  Fourth,
  Fifth,
  Sixth
}

/// Number of board ranks.
pub const NUM_RANKS: usize = 6;

/// Ranks list.
pub const ALL_RANKS: [Rank; NUM_RANKS] = [
  Rank::First,
  Rank::Second,
  Rank::Third,
  Rank::Fourth,
  Rank::Fifth,
  Rank::Sixth
];

impl Rank {
  /// Convert a rank number to a Rank.
  ///
  /// If the argument index is greater than the number of ranks, return the first
  /// rank.
  pub fn from_index(i: usize) -> Rank {
    unsafe {
      *ALL_RANKS.get_unchecked(i % NUM_RANKS)
    }
  }

  /// Get the rank number of the Rank.
  pub fn index(&self) -> usize {
    *self as usize
  }

  /// Go down one rank and wrap.
  pub fn down(&self) -> Rank {
    // TODO: Fix handling of special case.
    if self.index() == 0 {
      return Rank::from_index(NUM_RANKS - 1);
    }

    Rank::from_index(self.index().wrapping_sub(1))
  }

  /// Go up on rank and wrap.
  pub fn up (&self) -> Rank {
    Rank::from_index(self.index() + 1)
  }
}

#[cfg(test)]
mod tests {
  use super::{Rank, NUM_RANKS, ALL_RANKS};

  #[test]
  fn num_ranks() {
    assert_eq!(6, NUM_RANKS);
  }

  #[test]
  fn all_ranks() {
    assert_eq!(ALL_RANKS, [
      Rank::First,
      Rank::Second,
      Rank::Third,
      Rank::Fourth,
      Rank::Fifth,
      Rank::Sixth
    ]);
  }

  #[test]
  fn from_index() {
    assert_eq!(Rank::First,   Rank::from_index(0));
    assert_eq!(Rank::Second,  Rank::from_index(1));
    assert_eq!(Rank::Third,   Rank::from_index(2));
    assert_eq!(Rank::Fourth,  Rank::from_index(3));
    assert_eq!(Rank::Fifth,   Rank::from_index(4));
    assert_eq!(Rank::Sixth,   Rank::from_index(5));
    assert_eq!(Rank::First,   Rank::from_index(6));
    assert_eq!(Rank::Second,  Rank::from_index(7));
  }

  #[test]
  fn index() {
    assert_eq!(0, Rank::First.index());
    assert_eq!(1, Rank::Second.index());
    assert_eq!(2, Rank::Third.index());
    assert_eq!(3, Rank::Fourth.index());
    assert_eq!(4, Rank::Fifth.index());
    assert_eq!(5, Rank::Sixth.index());
  }

  #[test]
  fn down() {
    assert_eq!(Rank::First,   Rank::Second.down());
    assert_eq!(Rank::Second,  Rank::Third.down());
    assert_eq!(Rank::Third,   Rank::Fourth.down());
    assert_eq!(Rank::Fourth,  Rank::Fifth.down());
    assert_eq!(Rank::Fifth,   Rank::Sixth.down());
    assert_eq!(Rank::Sixth,   Rank::First.down());
  }

  #[test]
  fn up() {
    assert_eq!(Rank::First,   Rank::Sixth.up());
    assert_eq!(Rank::Second,  Rank::First.up());
    assert_eq!(Rank::Third,   Rank::Second.up());
    assert_eq!(Rank::Fourth,  Rank::Third.up());
    assert_eq!(Rank::Fifth,   Rank::Fourth.up());
    assert_eq!(Rank::Sixth,   Rank::Fifth.up());
  }
}
