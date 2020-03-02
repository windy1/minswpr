use super::Render;
use crate::config::ControlConfig;
use crate::fonts::Fonts;
use crate::layout::ComponentMap;
use crate::math::{Dimen, Point};
use sdl2::render::WindowCanvas;
use std::rc::Rc;

#[derive(Layout)]
pub struct RenderControl<'a> {
    #[allow(dead_code)]
    fonts: Rc<Fonts<'a>>,
    dimen: Dimen,
    components: ComponentMap<'a>,
    #[allow(dead_code)]
    config: ControlConfig,
}

impl Render for RenderControl<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
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
            components: Default::default(),
            config,
        }
    }
}
