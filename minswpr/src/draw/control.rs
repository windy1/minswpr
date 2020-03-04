use super::{Draw, DrawRect, Margins};
use crate::config::{ControlConfig, LedDisplayConfig};
use crate::draw::text::TextResult;
use crate::draw::text::{self, Text};
use crate::draw::DrawContext;
use crate::layout::{Layout, LayoutBuilder, Orientation};
use crate::math::{Dimen, Point};
use crate::utils;
use crate::BoardRef;
use crate::MsResult;
use sdl2::rect::Rect;
use std::rc::Rc;

use self::LedDisplayKind::*;

#[derive(new, AsAny)]
pub struct DrawLedDisplay {
    kind: LedDisplayKind,
    config: LedDisplayConfig,
}

pub enum LedDisplayKind {
    FlagCounter { board: BoardRef },
    Stopwatch,
}

impl Draw for DrawLedDisplay {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        self.draw_text(ctx, pos)
    }

    fn dimen(&self) -> Dimen {
        self.config.dimen
    }
}

impl DrawLedDisplay {
    fn draw_text(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        let text = self.make_text(ctx)?;
        let tq = text.query();
        let pos = pos + point!(self.dimen().width(), 0).as_i32() - point!(tq.width, 0).as_i32();

        ctx.canvas().copy(
            text.texture(),
            None,
            Some(Rect::new(pos.x, pos.y, tq.width, tq.height)),
        )
    }

    fn make_text<'a>(&self, ctx: &'a DrawContext<'a>) -> TextResult<'a> {
        text::make_text(ctx, match &self.kind {
            FlagCounter { board } => {
                let flags_remaining =
                    utils::borrow_safe(&board, |b| b.num_mines() as i32 - b.count_flags() as i32);
                Text::new(flags_remaining, "control.flag_counter", color!(red))
            }
            Stopwatch => Text::new(0, "control.stopwatch", color!(red)),
        })
    }
}

pub fn make_layout(config: &ControlConfig, board_width: u32, board: &BoardRef) -> MsResult<Layout> {
    let p = config.padding;

    let mut layout = LayoutBuilder::default()
        .color(Some(config.color))
        .padding(p)
        .orientation(Orientation::Horizontal)
        .build()?;

    let btn_dimen = config.reset_button_dimen;
    let w = board_width;
    let btn_width = btn_dimen.width();
    let fc = &config.flag_counter;
    let sw = &config.stopwatch;

    let btn_left = w / 2 - btn_width / 2 - fc.dimen.width() - p - fc.padding * 2;
    let btn_right = w / 2 - btn_width / 2 - sw.dimen.width() - p - sw.padding * 2;

    layout.insert_all(vec![
        (
            "flag_counter",
            Box::new(self::make_led_display(
                FlagCounter {
                    board: Rc::clone(board),
                },
                &fc,
            )?),
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
            Box::new(self::make_led_display(Stopwatch, &sw)?),
        ),
    ]);

    Ok(layout)
}

pub fn make_led_display(kind: LedDisplayKind, config: &LedDisplayConfig) -> MsResult<Layout> {
    let mut layout = LayoutBuilder::default()
        .color(Some(config.color))
        .padding(config.padding)
        .build()?;

    let text = Box::new(DrawLedDisplay::new(kind, config.clone()));
    layout.insert("text", 0, text);

    Ok(layout)
}
