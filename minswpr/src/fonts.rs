use crate::config::FontsConfig;
use crate::MsResult;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;

type FontMap<'a> = HashMap<String, Font<'a, 'a>>;

/// Handles font loading and storage
#[derive(new)]
pub struct Fonts<'a> {
    ttf: &'a Sdl2TtfContext,
    #[new(default)]
    font_map: FontMap<'a>,
}

impl<'a> Fonts<'a> {
    /// Creates a new Fonts and loads the fonts from the specified `FontsConfig`
    pub fn from_config(config: &FontsConfig, ttf: &'a Sdl2TtfContext) -> MsResult<Self> {
        let mut font_map = FontMap::new();
        for (k, f) in config {
            font_map.insert(k.to_string(), ttf.load_font(&f.path, f.pt)?);
        }
        Ok(Fonts { ttf, font_map })
    }

    /// Loads a new font from the specified `Path` with the specified size.
    ///
    /// # Arguments
    /// * `key` - Unique identifier of font
    /// * `fname` - Path to TTF file
    /// * `pt` - Font point size
    pub fn load(&mut self, key: &str, fname: &Path, size: u16) -> MsResult {
        self.font_map
            .insert(key.to_string(), self.ttf.load_font(fname, size)?);
        Ok(())
    }
}

impl<'a> Index<&str> for Fonts<'a> {
    type Output = Font<'a, 'a>;

    fn index(&self, key: &str) -> &Self::Output {
        self.font_map
            .get(key)
            .unwrap_or_else(|| panic!("missing required font `{}`", key))
    }
}
