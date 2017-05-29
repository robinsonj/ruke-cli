pub mod summon;

use duke::square::Square;

#[derive(Copy, Clone, Debug)]
pub struct MoveType {
  source:       Option<Square>,
  destination:  Option<Square>,
  remove:       Option<Square>,
  flip:         Option<Square>
}

pub trait Execute {
  fn execute(&self) -> bool;
}
