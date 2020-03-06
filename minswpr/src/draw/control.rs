use super::Draw;
use crate::config::LedDisplayConfig;
use crate::draw::text::TextResult;
use crate::draw::text::{self, Text};
use crate::draw::DrawContext;
use crate::math::{Dimen, Point};
use crate::utils;
use crate::{BoardRef, MsResult, StopwatchRef};
use sdl2::rect::Rect;
use std::cmp;

#[derive(new, AsAny)]
pub struct DrawLedDisplay {
    kind: LedDisplayKind,
    config: LedDisplayConfig,
}

pub enum LedDisplayKind {
    FlagCounter { board: BoardRef },
    Stopwatch { stopwatch: StopwatchRef },
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
        let normal_val = |i| cmp::max(-99, cmp::min(999, i));
        text::make_text(ctx, match &self.kind {
            LedDisplayKind::FlagCounter { board } => {
                let flags_remaining =
                    utils::borrow_safe(&board, |b| b.num_mines() as i32 - b.count_flags() as i32);
                Text::new(
                    normal_val(flags_remaining),
                    "control.flag_counter",
                    color!(red),
                )
            }
            LedDisplayKind::Stopwatch { stopwatch } => Text::new(
                normal_val(stopwatch.borrow().elapsed().as_secs() as i32),
                "control.stopwatch",
                color!(red),
            ),
        })
    }
}
