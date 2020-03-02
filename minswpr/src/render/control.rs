use super::Render;
use crate::config::ControlConfig;
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use sdl2::render::WindowCanvas;
use std::rc::Rc;

pub struct RenderControl<'ttf> {
    #[allow(dead_code)]
    fonts: Rc<Fonts<'ttf>>,
    dimen: Dimen,
    #[allow(dead_code)]
    config: ControlConfig,
}

impl Render for RenderControl<'_> {
    fn render(&self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        render_rect!(self.dimen, self.config.color, canvas, pos)
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }
}

impl<'ttf> RenderControl<'ttf> {
    pub fn new(fonts: Rc<Fonts<'ttf>>, config: ControlConfig, board_width: u32) -> Self {
        let dimen = point!(board_width, config.height);
        Self {
            fonts,
            dimen,
            config,
        }
    }
}
