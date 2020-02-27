use crate::math;
use sdl2::pixels::Color;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub fonts: HashMap<String, FontConfig>,
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
    pub flags: FlagsConfig,
}

#[derive(Deserialize)]
pub struct MinesConfig {
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub width: u32,
    pub height: u32,
}

pub type FlagsConfig = MinesConfig;

#[derive(Deserialize)]
pub struct FontConfig {
    pub path: PathBuf,
    pub pt: u16,
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
            let value = normalize_hex_str(value).map_err(de::Error::custom)?;
            let (r, g, b) = math::hex_to_rgb(value).map_err(de::Error::custom)?;
            println!("(r, g, b) = {:?}", (r, g, b));
            Ok(Color::RGB(r, g, b))
        }
    }

    des.deserialize_str(ColorVisitor)
}

fn normalize_hex_str(hex: &str) -> Result<&str, String> {
    let len = hex.len();
    if len == 0 {
        Err("cannot accept empty string for hex".to_string())
    } else if hex.as_bytes()[0] == b'#' {
        Ok(&hex[1..])
    } else {
        Ok(hex)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_normalize_hex_str_empty_str() {
        super::normalize_hex_str("").unwrap();
    }

    #[test]
    fn test_normalize_hex_str() -> Result<(), String> {
        assert_eq!("ffffff", super::normalize_hex_str("#ffffff")?);
        assert_eq!("ffffff", super::normalize_hex_str("ffffff")?);
        Ok(())
    }
}
