use duke::board::{Board};
use duke::color::{Color};
use duke::move_type::{MoveType};

pub mod ai;
pub mod ai_random;

pub trait Player {
  /// Return the color assigned to the player.
  fn color(&self) -> Color;
  fn turn(&self);
  fn setup_duke(&self, &Board) -> Option<MoveType>;
}
