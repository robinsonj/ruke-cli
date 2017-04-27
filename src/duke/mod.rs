#[derive(Debug)]
pub enum GameState {
  GameStart,
  GameRun
}

pub struct Game {
  state: GameState
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: GameState::GameStart
    }
  }

  pub fn run(&mut self) {
    println!("Hello world!");

    loop {
      match self.state {
        GameState::GameStart => {
          self.setup();
          self.state = GameState::GameRun;
        },
        GameState::GameRun => {
          println!("Running game!");
          break;
        },
        // _ => ()
      }
    }
  }

  pub fn setup(&mut self) {

  }
}
