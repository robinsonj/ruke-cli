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
    Tile::new(
      self.name.as_ref().unwrap().clone(),
      self.a.unwrap().clone(),
      self.b.unwrap().clone()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::{Config};
  use duke::tile::Tile;

  #[test]
  fn parse() {
    let footman: Config = match Config::parse("data/tiles/footman.toml") {
      Ok(c)   => c,
      Err(e)  => panic!("Tile parse error: {:?}", e)
    };

    assert_eq!("Footman", footman.tile().name);
  }

  fn tile() {
    let footman: Config = match Config::parse("data/tiles/footman.toml") {
      Ok(c)   => c,
      Err(e)  => panic!("Tile parse error: {:?}", e)
    };

    let tile: Tile = footman.tile();

    assert_eq!(tile.name, "Footman");
    assert_eq!(tile.a.x(), 2);
    assert_eq!(tile.a.y(), 2);
    assert_eq!(tile.b.x(), 2);
    assert_eq!(tile.b.y(), 2);
  }
}
