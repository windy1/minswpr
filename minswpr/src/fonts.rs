use crate::config::FontsConfig;
use crate::MsResult;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::path::Path;

type FontMap<'a> = HashMap<String, Font<'a, 'a>>;

#[derive(new)]
pub struct Fonts<'a> {
    ttf: &'a Sdl2TtfContext,
    #[new(default)]
    font_map: FontMap<'a>,
}

impl<'a> Fonts<'a> {
    pub fn from_config(config: &FontsConfig, ttf: &'a Sdl2TtfContext) -> MsResult<Self> {
        let mut font_map = FontMap::new();
        for (k, f) in config {
            font_map.insert(k.to_string(), ttf.load_font(&f.path, f.pt)?);
        }
        Ok(Fonts { ttf, font_map })
    }

    pub fn load(&mut self, key: &str, fname: &Path, size: u16) -> MsResult {
        self.font_map
            .insert(key.to_string(), self.ttf.load_font(fname, size)?);
        Ok(())
    }

    pub fn load_from_config(&mut self, config: &FontsConfig) -> MsResult {
        for (k, f) in config {
            self.load(k, &f.path, f.pt)?;
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> MsResult<&Font> {
        self.font_map
            .get(key)
            .ok_or_else(|| format!("missing required font `{}`", key))
    }
}
