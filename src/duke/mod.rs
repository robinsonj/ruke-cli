mod board;
mod space;
mod tile;
mod player;

#[derive(Debug)]
pub enum GameState {
  GameStart,
  GameRun,
  GameOver
}

pub struct Game {
  state: GameState,
  board: board::Board,
  blue:  Option<Box<player::Player>>,
  pink:  Option<Box<player::Player>>
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
      self.print_game();
      match self.state {
        GameState::GameStart => {
          self.setup();
          self.state = GameState::GameRun;
        },
        GameState::GameRun => {
          self.state = GameState::GameOver;
        },
        GameState::GameOver => {
          break;
        }
        // _ => ()
      }
    }
  }

  pub fn print_game(&mut self) {
    println!("Game[state]: {:?}", self.state);
    println!("Game[board]: {}", self.board);
  }

  fn setup(&mut self) {
    self.board.init();
  }
}
