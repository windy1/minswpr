use crate::RebaResult;
use crate::fonts::{Fonts, FontData};
use crate::context::Context;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub type Textures = TextureCreator<WindowContext>;

#[derive(new)]
pub struct DrawContext<'a> {
    context: &'a Context,
    canvas: WindowCanvas,
    fonts: &'a mut Fonts<'a>,
    textures: Textures,
}

impl<'a> DrawContext<'a> {
    pub fn canvas(&self) -> &WindowCanvas {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }

    pub fn load_font(&mut self, font: &FontData) -> RebaResult {
        self.fonts.load(&font.key, &font.fname, font.size)
    }
}
