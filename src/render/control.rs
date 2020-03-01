use super::Render;
use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::ControlConfig;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::rc::Rc;

pub struct RenderControl<'ttf> {
    #[allow(dead_code)]
    fonts: Rc<Fonts<'ttf>>,
    dimen: Dimen,
    color: Color,
    #[allow(dead_code)]
    config: ControlConfig,
}

impl<'ttf> Render for RenderControl<'ttf> {
    fn render(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        self.draw_base(canvas, pos)
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }
}

impl<'ttf> RenderControl<'ttf> {
    fn draw_base(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(
            pos.x,
            pos.y,
            self.dimen.width(),
            self.dimen.height(),
        ))
    }
}

#[derive(Default)]
pub struct RenderControlBuilder<'ttf> {
    fonts: Option<Rc<Fonts<'ttf>>>,
    dimen: Dimen,
    color: Option<Color>,
    config: Option<ControlConfig>,
}

impl<'ttf> RenderControlBuilder<'ttf> {
    pub fn fonts(mut self, fonts: Rc<Fonts<'ttf>>) -> Self {
        self.fonts = Some(fonts);
        self
    }

    pub fn board_width(mut self, board_width: u32) -> Self {
        self.dimen.set_width(board_width);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn config(mut self, config: ControlConfig) -> Self {
        self.dimen.set_height(config.height);
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<RenderControl<'ttf>, String> {
        let r = RenderControl {
            fonts: match self.fonts {
                Some(f) => f,
                None => return Err("`fonts` must be initialized".to_string()),
            },
            dimen: self.dimen,
            color: match self.color {
                Some(c) => c,
                None => return Err("`color` must be initialized".to_string()),
            },
            config: match self.config {
                Some(c) => c,
                None => return Err("`config` must be initialized".to_string()),
            },
        };
        Ok(r)
    }
}
