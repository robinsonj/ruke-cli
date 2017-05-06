pub mod ai;
pub mod ai_random;

pub trait Player {
  fn init(&self) -> Player;
}
