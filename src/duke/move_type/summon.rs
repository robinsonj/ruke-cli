use super::{Execute, MoveType};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct Summon {

}

impl Summon {
  pub fn new() -> MoveType {
    MoveType {
      source:       None,
      destination:  None,
      remove:       None,
      flip:         None
    }
  }
}

impl Execute for Summon {
  fn execute(&self) -> bool {
    true
  }
}
