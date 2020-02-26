use crate::render::colors;
use sdl2::pixels::Color;
use serde::{Deserialize, Deserializer};
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    #[serde(deserialize_with = "read_color")]
    pub bg_color: Color,
}

pub fn read_config<P>(fname: P) -> Result<Config, String>
where
    P: AsRef<Path>,
{
    let s = fs::read_to_string(fname).map_err(|e| e.to_string())?;
    Ok(toml::from_str(&s).map_err(|e| e.to_string())?)
}

fn read_color<'a, D>(_des: D) -> Result<Color, D::Error>
where
    D: Deserializer<'a>,
{
    // TODO
    Ok(colors::BLACK)
}
