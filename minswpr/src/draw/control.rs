use super::{Draw, DrawRect, Margins};
use crate::config::{ControlConfig, LedDisplayConfig};
use crate::draw::DrawContext;
use crate::layout::{Layout, LayoutBuilder, Orientation};
use crate::math::{Dimen, Point};

#[derive(new, AsAny)]
pub struct DrawLedDisplay {
    config: LedDisplayConfig,
}

impl Draw for DrawLedDisplay {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> Result<(), String> {
        render_rect!(self.dimen(), self.config.color, ctx, pos)
    }

    fn dimen(&self) -> Dimen {
        self.config.dimen
    }
}

impl DrawLedDisplay {
    fn draw_text(&mut self, _ctx: &DrawContext, _pos: Point) -> Result<(), String> {
        Ok(())
    }
}

pub fn make_layout(config: &ControlConfig, board_width: u32) -> Layout {
    let p = config.padding;

    let mut layout = LayoutBuilder::default()
        .color(Some(config.color))
        .padding(p)
        .orientation(Orientation::Horizontal)
        .build()
        .unwrap();

    let btn_dimen = config.reset_button_dimen;
    let w = board_width;
    let btn_width = btn_dimen.width();
    let btn_left = w / 2 - btn_width / 2 - config.flag_counter.dimen.width() - p;
    let btn_right = w / 2 - btn_width / 2 - config.stopwatch.dimen.width() - p;

    layout.insert_all(vec![
        (
            "flag_counter",
            Box::new(DrawLedDisplay::new(config.flag_counter.clone())),
        ),
        (
            "reset_button",
            Box::new(DrawRect::with_margins(
                btn_dimen,
                config.reset_button_color,
                *Margins::new().left(btn_left).right(btn_right),
            )),
        ),
        (
            "stopwatch",
            Box::new(DrawLedDisplay::new(config.stopwatch.clone())),
        ),
    ]);

    layout
}
