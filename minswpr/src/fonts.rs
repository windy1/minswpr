use crate::FontConfig;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::path::Path;

#[derive(new)]
pub struct Fonts<'ttf> {
    ttf: &'ttf Sdl2TtfContext,
    #[new(default)]
    font_map: HashMap<String, Font<'ttf, 'ttf>>,
}

impl<'ttf> Fonts<'ttf> {
    pub fn load(&mut self, key: &str, fname: &Path, size: u16) -> Result<(), String> {
        let font = self.ttf.load_font(fname, size)?;
        self.font_map.insert(key.to_string(), font);
        Ok(())
    }

    pub fn load_from_config(&mut self, config: &HashMap<String, FontConfig>) -> Result<(), String> {
        for (k, f) in config {
            self.load(k, &f.path, f.pt)?;
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<&Font, String> {
        self.font_map
            .get(key)
            .ok_or_else(|| format!("missing required font `{}`", key))
    }
}
