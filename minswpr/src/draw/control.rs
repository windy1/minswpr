use super::{DrawRect, Margins};
use crate::config::ControlConfig;
use crate::layout::{Layout, LayoutBuilder, Orientation};

pub fn make_layout(config: &ControlConfig, board_width: u32) -> Layout {
    let p = config.padding;

    let mut layout = LayoutBuilder::default()
        .color(Some(config.color))
        .padding(p)
        .orientation(Orientation::Horizontal)
        .build()
        .unwrap();

    let btn_dimen = config.reset_button_dimen;
    let btn_width = btn_dimen.width();
    let flag_counter_dimen = config.flag_counter_dimen;
    let stopwatch_dimen = config.stopwatch_dimen;

    let w = board_width;

    let btn_left = w / 2 - btn_width / 2 - flag_counter_dimen.width() - p;
    let btn_right = w / 2 - btn_width / 2 - stopwatch_dimen.width() - p;

    layout.insert_all(vec![
        (
            "flag_counter",
            Box::new(DrawRect::new(flag_counter_dimen, config.flag_counter_color)),
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
            Box::new(DrawRect::new(stopwatch_dimen, config.stopwatch_color)),
        ),
    ]);

    layout
}
