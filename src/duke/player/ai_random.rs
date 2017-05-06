use duke::player;
use duke::player::ai;

#[derive(Debug)]
pub struct RandomAIPlayer {
  name: String
}

impl RandomAIPlayer {

}

impl player::Player for RandomAIPlayer {
  fn init(&self) -> player::Player {
    RandomAIPlayer {
      name: "Random".to_string()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{RandomAIPlayer};

  #[test]
  fn init() {
    RandomAIPlayer::init();
    assert_eq!(true, false);
  }
}
