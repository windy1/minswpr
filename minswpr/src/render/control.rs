use super::Render;
use crate::config::ControlConfig;
use crate::fonts::Fonts;
use crate::layout::{self, ComponentMap, Layout, Orientation};
use crate::math::{Dimen, Point};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::rc::Rc;

pub struct RenderControl<'a> {
    #[allow(dead_code)]
    fonts: Rc<Fonts<'a>>,
    dimen: Dimen,
    components: ComponentMap<'a>,
    #[allow(dead_code)]
    config: ControlConfig,
}

impl<'a> RenderControl<'a> {
    pub fn new(fonts: Rc<Fonts<'a>>, config: ControlConfig, board_width: u32) -> Self {
        let dimen = point!(board_width, config.height);
        Self {
            fonts,
            dimen,
            components: Default::default(),
            config,
        }
    }
}

impl<'a> Layout<'a> for RenderControl<'a> {
    fn components(&self) -> &ComponentMap {
        &self.components
    }

    fn components_mut(&mut self) -> &mut ComponentMap<'a> {
        &mut self.components
    }

    fn color(&self) -> Option<Color> {
        Some(self.config.color)
    }

    fn orientation(&self) -> Orientation {
        Orientation::Horizontal
    }
}

impl Render for RenderControl<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        // render_rect!(self.dimen, self.config.color, canvas, pos)
        layout::do_render(self, canvas, pos)
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }
}
