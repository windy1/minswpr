use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::path::Path;

pub struct Fonts<'ttf> {
    ttf: &'ttf Sdl2TtfContext,
    font_map: HashMap<String, Font<'ttf, 'ttf>>, // TODO: &'static str
}

impl<'ttf> Fonts<'ttf> {
    pub fn new(ttf: &'ttf Sdl2TtfContext) -> Result<Self, String> {
        let font_map = HashMap::new();
        Ok(Self { ttf, font_map })
    }

    pub fn load(&mut self, key: &str, fname: &Path, size: u16) -> Result<(), String> {
        let font = self.ttf.load_font(fname, size)?;
        self.font_map.insert(key.to_string(), font);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<&Font<'ttf, 'ttf>, String> {
        self.font_map
            .get(key)
            .ok_or_else(|| format!("missing required font `{}`", key))
    }
}
