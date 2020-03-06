use crate::config::{ControlConfig, LedDisplayConfig};
use crate::draw::control::{DrawLedDisplay, DrawResetButtonBuilder, LedDisplayKind};
use crate::draw::Margins;
use crate::input;
use crate::layout::{Element, ElementBuilder, Layout, LayoutBuilder, Orientation};
use crate::{BoardRef, MsResult, ResetButtonRef, StopwatchRef};
use std::convert::TryInto;
use std::rc::Rc;

#[derive(Builder)]
pub struct ControlLayout<'a> {
    config: &'a ControlConfig,
    board_width: u32,
    board: &'a BoardRef,
    stopwatch: &'a StopwatchRef,
    reset_button: &'a ResetButtonRef,
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

        let btn_config = &self.config.reset_button;
        let btn_dimen = btn_config.dimen;
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
                    .draw_ref(Box::new(
                        DrawResetButtonBuilder::default()
                            .config(btn_config.clone())
                            .button(Rc::clone(&self.reset_button))
                            .margins(*Margins::new().left(btn_left).right(btn_right))
                            .build()?,
                    ))
                    .mouse_up(Box::new(input::control::on_reset_mouse_up))
                    .mouse_down(Box::new(input::control::on_reset_mouse_down))
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
