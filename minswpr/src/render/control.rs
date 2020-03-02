use super::{Margins, Render, RenderRect};
use crate::config::ControlConfig;
use crate::fonts::Fonts;
use crate::layout::{self, ComponentMap, Layout, Orientation, RenderRef};
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
        let mut r = Self {
            fonts,
            dimen: point!(board_width, config.height),
            components: Default::default(),
            config,
        };

        r.insert_all(r.make_components());

        r
    }

    fn make_components(&self) -> Vec<(&'static str, RenderRef<'a>)> {
        let w = self.dimen.width();
        let p = self.padding();

        let btn_dimen = self.config.reset_button_dimen;
        let btn_width = btn_dimen.width();
        let flag_counter_dimen = self.config.flag_counter_dimen;
        let stopwatch_dimen = self.config.stopwatch_dimen;

        let btn_left = w / 2 - btn_width / 2 - flag_counter_dimen.width() - p;
        let btn_right = w / 2 - btn_width / 2 - stopwatch_dimen.width() - p;

        vec![
            (
                "flag_counter",
                Box::new(RenderRect::new(
                    flag_counter_dimen,
                    self.config.flag_counter_color,
                )),
            ),
            (
                "reset_button",
                Box::new(RenderRect::with_margins(
                    btn_dimen,
                    self.config.reset_button_color,
                    *Margins::new().left(btn_left).right(btn_right),
                )),
            ),
            (
                "stopwatch",
                Box::new(RenderRect::new(
                    stopwatch_dimen,
                    self.config.stopwatch_color,
                )),
            ),
        ]
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

    fn padding(&self) -> u32 {
        self.config.padding
    }

    fn orientation(&self) -> Orientation {
        Orientation::Horizontal
    }
}

impl Render for RenderControl<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        layout::do_render(self, canvas, pos)
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }
}
