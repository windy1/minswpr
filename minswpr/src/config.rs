use crate::math::{self, Dimen};
use crate::MsResult;
use sdl2::pixels::Color;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref DEFAULT_CONFIG: &'static Path = Path::new("minswpr.toml");
}

/// Helper type for the configured fonts to load into `Fonts`
pub type FontsConfig = HashMap<String, FontConfig>;

/// Base config for application
#[derive(Deserialize, Clone)]
pub struct Config {
    pub window: WindowConfig,
    pub fonts: FontsConfig,
    pub control: ControlConfig,
    pub board: BoardConfig,
    pub layout: LayoutConfig,
}

/// Window specific values
#[derive(Deserialize, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub dimen: Dimen,
    #[serde(deserialize_with = "read_color")]
    pub bg_color: Color,
}

/// `Layout` specific values
#[derive(Deserialize, Clone)]
pub struct LayoutConfig {
    pub padding: u32,
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub guides: bool,
}

/// Values specific to the central control panel located on the top of the board
/// (by default)
#[derive(Deserialize, Clone)]
pub struct ControlConfig {
    pub height: u32,
    pub spacer_height: u32,
    #[serde(deserialize_with = "read_color")]
    pub spacer_color: Color,
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub padding: u32,
    pub reset_button: ButtonConfig,
    pub flag_counter: LedDisplayConfig,
    pub stopwatch: LedDisplayConfig,
}

#[derive(Deserialize, Clone)]
pub struct ButtonConfig {
    pub dimen: Dimen,
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    #[serde(deserialize_with = "read_color")]
    pub pressed_color: Color,
}

/// Config for LED display like the flag counter or stopwatch
#[derive(Deserialize, Clone)]
pub struct LedDisplayConfig {
    pub dimen: Dimen,
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub padding: u32,
}

/// `Board` specific values
#[derive(Deserialize, Clone)]
pub struct BoardConfig {
    pub dimen: Dimen<usize>,
    pub num_mines: usize,
    pub cells: CellConfig,
}

/// Values specific to the drawn cells on the board
#[derive(Deserialize, Clone)]
pub struct CellConfig {
    pub dimen: Dimen,
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    #[serde(deserialize_with = "read_color")]
    pub pressed_color: Color,
    pub border_width: u32,
    #[serde(deserialize_with = "read_color")]
    pub border_color: Color,
    #[serde(deserialize_with = "read_color")]
    pub revealed_color: Color,
    #[serde(deserialize_with = "read_color")]
    pub text_color: Color,
    pub mines: MinesConfig,
    pub flags: FlagsConfig,
}

/// Values specific to the look of mines
#[derive(Deserialize, Clone)]
pub struct MinesConfig {
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub dimen: Dimen,
    #[serde(deserialize_with = "read_color")]
    pub revealed_color: Color,
}

/// Values specific to the look of flags
#[derive(Deserialize, Clone)]
pub struct FlagsConfig {
    #[serde(deserialize_with = "read_color")]
    pub color: Color,
    pub dimen: Dimen,
}

/// Values specific for fonts
#[derive(Deserialize, Clone)]
pub struct FontConfig {
    pub path: PathBuf,
    pub pt: u16,
}

/// Reads a config file from the specified `Path` and returns `Ok(Config)` if
/// successful, `Err(String)` otherwise.
pub fn read_config<P>(fname: P) -> MsResult<Config>
where
    P: AsRef<Path>,
{
    let s = fs::read_to_string(fname).map_err(|e| e.to_string())?;
    Ok(toml::from_str(&s).map_err(|e| e.to_string())?)
}

/// Tries to resolve the config path. If there is a config named
/// `minswpr.{OS}.toml` that will be used over the default `minswpr.toml` file.
pub fn resolve() -> MsResult<PathBuf> {
    let for_os = |os: &str| -> PathBuf {
        let p = PathBuf::from(&format!("minswpr.{}.toml", os));
        if p.exists() {
            p
        } else {
            DEFAULT_CONFIG.to_path_buf()
        }
    };

    Ok(for_os(
        &sys_info::os_type()
            .map_err(|e| format!("could not resolve config file: `{}`", e))?
            .to_lowercase(),
    ))
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
