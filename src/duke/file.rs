/// Represent a board file.
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum File {
  A,
  B,
  C,
  D,
  E,
  F
}

/// Number of board files.
pub const NUM_FILES: usize = 6;

/// Files list.
pub const ALL_FILES: [File; NUM_FILES] = [
  File::A,
  File::B,
  File::C,
  File::D,
  File::E,
  File::F
];

impl File {
  /// Convert a file number to a File.
  pub fn from_index(i: usize) -> File {
    unsafe {
      *ALL_FILES.get_unchecked(i % NUM_FILES)
    }
  }

  /// Get the file number of the File.
  pub fn index(&self) -> usize {
    *self as usize
  }

  /// Go left one rank and wrap.
  pub fn left(&self) -> File {
    // TODO: Fix handling of special case.
    if self.index() == 0 {
      return File::from_index(NUM_FILES - 1);
    }

    File::from_index(self.index().wrapping_sub(1))
  }

  /// Go right one rank and wrap.
  pub fn right(&self) -> File {
    File::from_index(self.index() + 1)
  }
}

#[cfg(test)]
mod tests {
  use super::{File, NUM_FILES, ALL_FILES};

  #[test]
  fn num_files() {
    assert_eq!(6, NUM_FILES);
  }

  #[test]
  fn all_files() {
    assert_eq!(ALL_FILES, [
      File::A,
      File::B,
      File::C,
      File::D,
      File::E,
      File::F
    ]);
  }

  #[test]
  fn from_index() {
    assert_eq!(File::A, File::from_index(0));
    assert_eq!(File::B, File::from_index(1));
    assert_eq!(File::C, File::from_index(2));
    assert_eq!(File::D, File::from_index(3));
    assert_eq!(File::E, File::from_index(4));
    assert_eq!(File::F, File::from_index(5));
    assert_eq!(File::A, File::from_index(6));
    assert_eq!(File::B, File::from_index(7));
  }

  #[test]
  fn index() {
    assert_eq!(0, File::A.index());
    assert_eq!(1, File::B.index());
    assert_eq!(2, File::C.index());
    assert_eq!(3, File::D.index());
    assert_eq!(4, File::E.index());
    assert_eq!(5, File::F.index());
  }

  #[test]
  fn left() {
    assert_eq!(File::A, File::B.left());
    assert_eq!(File::B, File::C.left());
    assert_eq!(File::C, File::D.left());
    assert_eq!(File::D, File::E.left());
    assert_eq!(File::E, File::F.left());
    assert_eq!(File::F, File::A.left());
  }

  #[test]
  fn right() {
    assert_eq!(File::A, File::F.right());
    assert_eq!(File::B, File::A.right());
    assert_eq!(File::C, File::B.right());
    assert_eq!(File::D, File::C.right());
    assert_eq!(File::E, File::D.right());
    assert_eq!(File::F, File::E.right());
  }
}
