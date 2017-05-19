mod board;
mod space;
mod tile;
mod player;

use duke::player::Player;

#[derive(Debug)]
pub enum GameState {
  GameStart,
  GameRun,
  GameOver
}

pub struct Game {
  state: GameState,
  board: board::Board,
  blue:  Option<player::ai_random::Random>,
  pink:  Option<player::ai_random::Random>
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: GameState::GameStart,
      board: board::Board::new(),
      blue:  None,
      pink:  None
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

          match self.bp() {
            Ok(p) => p.turn(),
            Err(e) => println!("Error taking turn: {:?}", e)
          }

          match self.pp() {
            Ok(p) => p.turn(),
            Err(e) => println!("Error taking turn: {:?}", e)
          }
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
    // println!("Game[board] (DEBUG): {:?}", self.board);
  }

  fn setup(&mut self) {
    self.board.init();
    self.blue = Some(player::ai_random::Random::init());
    self.pink = Some(player::ai_random::Random::init());
  }

  fn bp(&self) -> Result<&player::ai_random::Random, String> {
    Ok(self.blue.as_ref().unwrap().clone())
  }

  fn pp(&mut self) -> Result<&player::ai_random::Random, String> {
    Ok(self.pink.as_ref().unwrap().clone())
  }
}
