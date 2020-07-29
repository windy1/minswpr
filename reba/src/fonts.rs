use crate::RebaResult;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::ops::Index;
use std::path::{Path, PathBuf};

type FontMap<'a> = HashMap<String, Font<'a, 'a>>;

pub struct FontData {
    pub key: String,
    pub fname: PathBuf,
    pub size: u16
}

/// Handles font loading and storage
#[derive(new)]
pub struct Fonts<'a> {
    ttf: &'a Sdl2TtfContext,
    #[new(default)]
    font_map: FontMap<'a>,
}

impl<'a> Fonts<'a> {
    /// Loads a new font from the specified `Path` with the specified size.
    ///
    /// # Arguments
    /// * `key` - Unique identifier of font
    /// * `fname` - Path to TTF file
    /// * `pt` - Font point size
    pub fn load(&mut self, key: &str, fname: &Path, size: u16) -> RebaResult {
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
