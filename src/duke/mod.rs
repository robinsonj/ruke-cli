mod board;
mod space;

#[derive(Debug)]
pub enum GameState {
  GameStart,
  GameRun,
  GameOver
}

pub struct Game {
  state: GameState,
  board: board::Board
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: GameState::GameStart,
      board: board::Board::new()
    }
  }

  pub fn run(&mut self) {

    loop {
      match self.state {
        GameState::GameStart => {
          self.setup();
          self.state = GameState::GameRun;
        },
        GameState::GameRun => {
          println!("Running game!");
          self.state = GameState::GameOver;
        },
        GameState::GameOver => {
          println!("Game over!");
          break;
        }
        // _ => ()
      }
    }
  }

  pub fn setup(&mut self) {

  }
}
