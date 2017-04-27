use super::space::*;

const NUM_SPACES: usize = 100;

pub struct Board {
  spaces: Vec<Space>
}

impl Board {
  pub fn new() -> Board {
    Board {
      spaces: Vec::with_capacity(NUM_SPACES)
    }
  }
}
