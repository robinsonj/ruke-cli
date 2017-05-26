extern crate toml;

use std;

use super::{Tile};

#[derive(Debug)]
enum Error {
  OpenError(std::io::Error),
  ReadError(std::io::Error),
  ParseError(toml::de::Error)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
pub struct Config {
  name:   Option<String>,
  startx: Option<u8>,
  starty: Option<u8>
}

impl Config {
  fn parse(filename: &str) -> Result<Config> {
    use std::io::Read;

    let mut f = std::fs::File::open(filename)
      .map_err(Error::OpenError)?;

    let mut s = String::new();

    f.read_to_string(&mut s)
      .map_err(Error::ReadError)?;

    toml::from_str(&s)
      .map_err(Error::ParseError)
  }

  fn tile(&self) -> Tile {
    // Tile::new(&mut self.name.unwrap(), self.startx.unwrap(), self.starty.unwrap())
    Tile {
      name: "test".to_string(),
      x:    2,
      y:    2
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Config};

  #[test]
  fn parse() {
    match Config::parse("data/tiles/footman.toml") {
      Ok(c) => println!("Tile: {:?}", c),
      Err(e) => println!("Error: {:?}", e)
    }
  }
}
