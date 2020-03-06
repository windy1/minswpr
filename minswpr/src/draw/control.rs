use super::Draw;
use crate::board::Board;
use crate::config::{ButtonConfig, LedDisplayConfig};
use crate::control::{Button, Stopwatch};
use crate::draw::text::TextResult;
use crate::draw::text::{self, Text};
use crate::draw::DrawContext;
use crate::draw::Margins;
use crate::math::{Dimen, Point};
use crate::{utils, Model, MsResult};
use sdl2::rect::Rect;
use std::cmp;

#[derive(Builder, AsAny)]
pub struct DrawResetButton {
    config: ButtonConfig,
    button: Model<Button>,
    margins: Margins,
}

impl Draw for DrawResetButton {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        let color = if self.button.borrow().is_pressed() {
            self.config.pressed_color
        } else {
            self.config.color
        };
        draw_rect!(self.dimen(), color, ctx, pos)
    }

    fn dimen(&self) -> Dimen {
        self.config.dimen
    }

    fn margins(&self) -> Margins {
        self.margins
    }
}

#[derive(new, AsAny)]
pub struct DrawLedDisplay {
    kind: LedDisplayKind,
    config: LedDisplayConfig,
}

pub enum LedDisplayKind {
    FlagCounter(Model<Board>),
    Stopwatch(Model<Stopwatch>),
}

impl Draw for DrawLedDisplay {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        let text = self.make_text(ctx)?;
        let tq = text.query();
        let pos = pos + point!(self.dimen().width(), 0).as_i32() - point!(tq.width, 0).as_i32();
        ctx.canvas().copy(
            text.texture(),
            None,
            Some(Rect::new(pos.x, pos.y, tq.width, tq.height)),
        )
    }

    fn dimen(&self) -> Dimen {
        self.config.dimen
    }
}

impl DrawLedDisplay {
    const MAX_VALUE: i32 = 999;
    const MIN_VALUE: i32 = -99;

    fn make_text<'a>(&self, ctx: &'a DrawContext<'a>) -> TextResult<'a> {
        let normal_val = |i| cmp::max(Self::MIN_VALUE, cmp::min(Self::MAX_VALUE, i));
        text::make_text(ctx, match &self.kind {
            LedDisplayKind::FlagCounter(board) => {
                let flags_remaining = utils::borrow_safe(&board.as_ref(), |b| {
                    b.num_mines() as i32 - b.count_flags() as i32
                });
                Text::new(
                    normal_val(flags_remaining),
                    "control.flag_counter",
                    color!(red),
                )
            }
            LedDisplayKind::Stopwatch(stopwatch) => Text::new(
                normal_val(stopwatch.borrow().elapsed().as_secs() as i32),
                "control.stopwatch",
                color!(red),
            ),
        })
    }
}
