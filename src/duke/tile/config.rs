extern crate toml;

use std;

use super::{Side, Tile};

#[derive(Debug)]
enum Error {
  OpenError(std::io::Error),
  ReadError(std::io::Error),
  ParseError(toml::de::Error)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
pub struct Config {
  name: Option<String>,
  a:    Option<Side>,
  b:    Option<Side>
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
    Tile::new(self.name.as_ref().unwrap().clone())
  }
}

#[cfg(test)]
mod tests {
  use super::{Config};

  #[test]
  fn parse() {
    let footman: Config = match Config::parse("data/tiles/footman.toml") {
      Ok(c)   => c,
      Err(e)  => panic!("Tile parse error: {:?}", e)
    };

    assert_eq!("Footman", footman.tile().name);
  }
}
