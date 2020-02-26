use crate::math;
use sdl2::pixels::Color;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub board: BoardConfig,
}

#[derive(Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    #[serde(deserialize_with = "read_color")]
    pub bg_color: Color,
}

#[derive(Deserialize)]
pub struct BoardConfig {
    pub width: usize,
    pub height: usize,
    pub mine_frequency: f64,
    pub cells: CellAttrsConfig,
}

#[derive(Deserialize)]
pub struct CellAttrsConfig {
    pub width: u32,
    pub height: u32,
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub border_width: u32,
    #[serde(deserialize_with = "read_color")]
    pub border_color: Color,
    #[serde(deserialize_with = "read_color")]
    pub revealed_color: Color,
    pub mines: MinesConfig,
}

#[derive(Deserialize)]
pub struct MinesConfig {
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub width: u32,
    pub height: u32,
}

pub fn read_config<P>(fname: P) -> Result<Config, String>
where
    P: AsRef<Path>,
{
    let s = fs::read_to_string(fname).map_err(|e| e.to_string())?;
    Ok(toml::from_str(&s).map_err(|e| e.to_string())?)
}

fn read_color<'de, D>(des: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    struct ColorVisitor;

    impl<'de> Visitor<'de> for ColorVisitor {
        type Value = Color;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "hex string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Color, E>
        where
            E: de::Error,
        {
            let value = normalize_hex_str(value)?;
            let (r, g, b) = math::hex_to_rgb(value);
            println!("(r, g, b) = {:?}", (r, g, b));
            Ok(Color::RGB(r, g, b))
        }
    }

    des.deserialize_str(ColorVisitor {})
}

fn normalize_hex_str<E>(hex: &str) -> Result<&str, E>
where
    E: de::Error,
{
    let len = hex.len();
    if len == 0 {
        Err(de::Error::invalid_length(
            len,
            &"cannot accept empty string for hex",
        ))
    } else if hex.as_bytes()[0] == b'#' {
        Ok(&hex[1..])
    } else {
        Ok(hex)
    }
}
