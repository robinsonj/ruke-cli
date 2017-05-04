extern crate toml;

use std::fs::File;
use std::io::prelude::*;

use super::{Tile};

#[derive(Debug, Deserialize)]
pub struct Config {
  name:   Option<String>,
  startx: Option<u8>,
  starty: Option<u8>
}

impl Config {
  pub fn parse(path: String) -> Tile {
    let mut config_toml = String::new();

    let mut file = match File::open(&path) {
      Ok(file) => file,
      Err(_) => {
        println!("Could not read config, using default.");
        return Tile::new("Dummy".to_string(), 0, 0);
      }
    };

    file.read_to_string(&mut config_toml)
      .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let parsed: Config = toml::from_str(config_toml.as_str()).unwrap();

    println!("{:?}", parsed);

    Tile {
      name: "dump".to_string(),
      x: 0,
      y: 0
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Config};

  #[test]
  fn parse() {
    let tile = Config::parse("data/tiles/footman.toml".to_string());
  }
}
