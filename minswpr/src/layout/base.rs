use super::{ComponentMap, Layout};
use crate::config::LayoutConfig;
use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

#[derive(new)]
pub struct LayoutBase<'a> {
    #[new(default)]
    components: ComponentMap<'a>,
    config: LayoutConfig,
}

impl<'a> Layout<'a> for LayoutBase<'a> {
    fn components(&self) -> &ComponentMap {
        &self.components
    }

    fn components_mut(&mut self) -> &mut ComponentMap<'a> {
        &mut self.components
    }

    fn color(&self) -> Option<Color> {
        Some(self.config.color)
    }

    fn padding(&self) -> u32 {
        self.config.padding
    }
}

impl Render for LayoutBase<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        super::do_render(self, canvas, pos)
    }

    fn dimen(&self) -> Dimen {
        super::calc_dimen(self)
    }
}
