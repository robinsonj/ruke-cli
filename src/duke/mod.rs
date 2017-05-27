mod board;
mod color;
mod file;
mod rank;
mod space;
mod square;
mod tile;
mod player;

use duke::player::Player;
use duke::color::Color;

#[derive(Debug)]
pub enum GameState {
  GameStart,
  GameRun,
  GameOver
}

pub struct Game {
  state: GameState,
  board: board::Board,
  turn:  Color,
  blue:  Option<player::ai_random::Random>,
  pink:  Option<player::ai_random::Random>
}

impl Game {
  pub fn new() -> Game {
    Game {
      state: GameState::GameStart,
      board: board::Board::new(),
      turn:  Color::Pink,
      blue:  None,
      pink:  None
    }
  }

  pub fn run(&mut self) {
    let mut turn_counter = 0;

    loop {
      self.print_game();
      match self.state {
        GameState::GameStart => {
          self.setup();
          self.state = GameState::GameRun;
        },
        GameState::GameRun => {
          turn_counter += 1;

          if turn_counter > 20 {
            self.state = GameState::GameOver;
          }

          match self.turn {
            Color::Blue=> {
              match self.bp() {
                Ok(p) => p.turn(),
                Err(e) => println!("Error taking turn: {:?}", e)
              };

              self.turn = Color::Pink;
            },
            Color::Pink => {
              match self.pp() {
                Ok(p) => p.turn(),
                Err(e) => println!("Error taking turn: {:?}", e)
              };

              self.turn = Color::Blue;
            }
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
    println!("Game[turn]:  {:?}", self.turn);
    println!("Game[board]: {}", self.board);
    // println!("Game[board] (DEBUG): {:?}", self.board);
  }

  fn setup(&mut self) {
    self.board.init();
    self.blue = Some(player::ai_random::Random::new(Color::Blue));
    self.pink = Some(player::ai_random::Random::new(Color::Pink));
  }

  fn bp(&self) -> Result<&player::ai_random::Random, String> {
    Ok(self.blue.as_ref().unwrap().clone())
  }

  fn pp(&mut self) -> Result<&player::ai_random::Random, String> {
    Ok(self.pink.as_ref().unwrap().clone())
  }
}
