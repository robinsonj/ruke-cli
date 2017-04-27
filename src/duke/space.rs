use std::fmt;

#[derive(Debug)]
pub enum SpaceType {
  OutOfBounds,
  Playable,
  BlueStart,
  PinkStart
}

pub struct Space {
  space_type: SpaceType
}

impl Space {
  pub fn new(space: SpaceType) -> Space {
    Space {
      space_type: space
    }
  }
}

impl fmt::Debug for Space {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let message = match self.space_type {
      SpaceType::Playable | SpaceType::BlueStart | SpaceType:: PinkStart => "_",
      _ => "X"
    };

    write!(f, "{}", message)
  }
}

impl fmt::Display for Space {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let message = match self.space_type {
      SpaceType::Playable | SpaceType::BlueStart | SpaceType:: PinkStart => "_",
      _ => "X"
    };

    write!(f, "{}", message)
  }
}
