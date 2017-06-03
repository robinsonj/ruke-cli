use std::ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign,
  Not};

/// Classic Bitboard.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct BitBoard(pub u64);

pub const EMPTY:  BitBoard  = BitBoard(0x0000000000000000u64);
pub const FULL:   BitBoard  = BitBoard(0xFFFFFFFFFFFFFFFFu64);

impl Not for BitBoard {
  type Output = BitBoard;

  fn not(self) -> BitBoard {
    BitBoard(!self.0)
  }
}

impl BitAnd for BitBoard {
  type Output = BitBoard;

  fn bitand(self, other: BitBoard) -> BitBoard {
    BitBoard(self.0 & other.0)
  }
}

impl BitOr for BitBoard {
  type Output = BitBoard;

  fn bitor(self, other: BitBoard) -> BitBoard {
    BitBoard(self.0 | other.0)
  }
}

impl BitXor for BitBoard {
  type Output = BitBoard;

  fn bitxor(self, other: BitBoard) -> BitBoard {
    BitBoard(self.0 ^ other.0)
  }
}

impl BitAndAssign for BitBoard {
  fn bitand_assign(&mut self, other: BitBoard) {
    self.0 &= other.0;
  }
}

impl BitOrAssign for BitBoard {
  fn bitor_assign(&mut self, other: BitBoard) {
    self.0 |= other.0;
  }
}

impl BitXorAssign for BitBoard {
  fn bitxor_assign(&mut self, other: BitBoard) {
    self.0 ^= other.0;
  }
}

#[cfg(test)]
mod tests {
  use super::{BitBoard, EMPTY, FULL};

  #[test]
  fn empty() {
    assert_eq!(EMPTY, BitBoard(<u64>::min_value()));
  }

  #[test]
  fn full() {
    assert_eq!(FULL, BitBoard(<u64>::max_value()));
  }

  #[test]
  fn not() {
    assert_eq!(FULL, !EMPTY);
  }

  #[test]
  fn bitand() {
    assert_eq!(EMPTY,                   EMPTY & BitBoard(<u64>::max_value()));
    assert_eq!(BitBoard(1),             BitBoard(1) & BitBoard(101));
    assert_eq!(BitBoard(0b00000001u64), BitBoard(0b00000001u64) & BitBoard(0b01010101u64));
  }

  #[test]
  fn bitor() {
    assert_eq!(FULL,                    EMPTY | FULL);
    assert_eq!(BitBoard(0b11111111u64), BitBoard(0b10101010u64) | BitBoard(0b01010101u64));
  }

  #[test]
  fn bitxor() {
    assert_eq!(FULL,                    EMPTY ^ FULL);
    assert_eq!(EMPTY,                   FULL ^ FULL);
  }

  #[test]
  fn bitand_assign() {
    let mut board = BitBoard(1);
    board &= BitBoard(1);
    assert_eq!(board, BitBoard(1));
  }

  #[test]
  fn bitor_assign() {
    let mut board = BitBoard(3);
    board |= BitBoard(4);
    assert_eq!(board, BitBoard(7));
  }

  #[test]
  fn bitxor_assign() {
    let mut board = BitBoard(1);
    board ^= BitBoard(3);
    assert_eq!(board, BitBoard(2));
  }
}
