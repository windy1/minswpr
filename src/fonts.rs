use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::ops;
use std::path::Path;

pub struct Fonts<'a> {
    ttf: &'a Sdl2TtfContext,
    font_map: HashMap<String, Font<'a, 'a>>,
}

impl<'a> Fonts<'a> {
    pub fn new(ttf: &'a Sdl2TtfContext) -> Result<Self, String> {
        let font_map = HashMap::new();
        Ok(Self { ttf, font_map })
    }

    pub fn load(&mut self, key: &str, fname: &Path, size: u16) -> Result<(), String> {
        let font = self.ttf.load_font(fname, size)?;
        self.font_map.insert(key.to_string(), font);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&Font<'a, 'a>> {
        self.font_map.get(key)
    }
}

impl<'a> ops::Index<&str> for Fonts<'a> {
    type Output = Font<'a, 'a>;

    fn index(&self, key: &str) -> &Self::Output {
        &self.font_map[key]
    }
}
