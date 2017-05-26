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
}
