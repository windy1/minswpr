use crate::board::Board;
use crate::config::{ControlConfig, LedDisplayConfig};
use crate::control::{Button, Stopwatch};
use crate::draw::control::{DrawLedDisplay, DrawResetButtonBuilder, LedDisplayKind};
use crate::draw::Margins;
use crate::input;
use crate::layout::{Element, ElementBuilder, Layout, LayoutBuilder, Orientation};
use crate::{Model, MsResult};
use std::convert::TryInto;

#[derive(Builder)]
pub struct ControlLayout<'a> {
    config: &'a ControlConfig,
    board_width: u32,
    board: &'a Model<Board>,
    stopwatch: &'a Model<Stopwatch>,
    reset_button: &'a Model<Button>,
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
                    LedDisplayKind::FlagCounter(self.board.clone()),
                    &fc,
                )?)),
            ),
            (
                "reset_button",
                ElementBuilder::default()
                    .draw_ref(Box::new(
                        DrawResetButtonBuilder::default()
                            .config(btn_config.clone())
                            .button(self.reset_button.clone())
                            .margins(*Margins::new().left(btn_left).right(btn_right))
                            .build()?,
                    ))
                    .mouse_up(Box::new(input::control::on_mouse_up_reset_button))
                    .mouse_down(Box::new(input::control::on_mouse_down_reset_button))
                    .mouse_leave(Box::new(input::control::on_mouse_leave_reset_button))
                    .mouse_enter(Box::new(input::control::on_mouse_enter_reset_button))
                    .build()?,
            ),
            (
                "stopwatch",
                Element::new(Box::new(self::make_led_display(
                    LedDisplayKind::Stopwatch(self.stopwatch.clone()),
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
