pub struct Random {}

impl Random {
  pub fn init() -> Random {
    Random {}
  }
}

impl super::Player for Random {
  fn turn(&self) {
    println!("Random player taking turn")
  }
}

impl super::ai::AIPlayer for Random {

}
