use crate::config::{ControlConfig, LedDisplayConfig};
use crate::draw::control::{DrawLedDisplay, LedDisplayKind};
use crate::draw::{DrawRect, Margins};
use crate::layout::{Element, ElementBuilder, Layout, LayoutBuilder, Orientation};
use crate::{BoardRef, GameState, MsResult, StopwatchRef};
use std::convert::TryInto;
use std::rc::Rc;

#[derive(Builder)]
pub struct ControlLayout<'a> {
    config: &'a ControlConfig,
    board_width: u32,
    board: &'a BoardRef,
    stopwatch: &'a StopwatchRef,
}

impl TryInto<Layout> for ControlLayout<'_> {
    type Error = String;

    fn try_into(self) -> MsResult<Layout> {
        let p = self.config.padding;

        let mut layout = LayoutBuilder::default()
            .color(self.config.color)
            .padding(p)
            .orientation(Orientation::Horizontal)
            .build()?;

        let btn_dimen = self.config.reset_button_dimen;
        let w = self.board_width;
        let btn_width = btn_dimen.width();
        let fc = &self.config.flag_counter;
        let sw = &self.config.stopwatch;

        let btn_left = w / 2 - btn_width / 2 - fc.dimen.width() - p - fc.padding * 2;
        let btn_right = w / 2 - btn_width / 2 - sw.dimen.width() - p - sw.padding * 2;

        layout.insert_all(vec![
            (
                "flag_counter",
                Element::new(Box::new(self::make_led_display(
                    LedDisplayKind::FlagCounter {
                        board: Rc::clone(self.board),
                    },
                    &fc,
                )?)),
            ),
            (
                "reset_button",
                ElementBuilder::default()
                    .draw_ref(Box::new(DrawRect::with_margins(
                        btn_dimen,
                        self.config.reset_button_color,
                        *Margins::new().left(btn_left).right(btn_right),
                    )))
                    .mouse_up(Box::new(|_, _| GameState::Reset))
                    .build()?,
            ),
            (
                "stopwatch",
                Element::new(Box::new(self::make_led_display(
                    LedDisplayKind::Stopwatch {
                        stopwatch: Rc::clone(self.stopwatch),
                    },
                    &sw,
                )?)),
            ),
        ]);

        Ok(layout)
    }
}

fn make_led_display(kind: LedDisplayKind, config: &LedDisplayConfig) -> MsResult<Layout> {
    let mut layout = LayoutBuilder::default()
        .color(config.color)
        .padding(config.padding)
        .build()?;

    let text = Element::new(Box::new(DrawLedDisplay::new(kind, config.clone())));
    layout.insert("text", 0, text);

    Ok(layout)
}
