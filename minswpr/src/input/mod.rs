pub mod board;
pub mod control;
pub mod events;

use crate::layout::Element;

use self::events::*;

/// Returns an `OnMouse<E: MouseEvent>` handler that will defer `MouseUpEvent`s
/// to the specified `Layout`'s child elements. Panics if the `Node` with
/// `layout_id` is not a `Layout`
pub fn defer_mouse<E, F>(layout_id: &'static str, f: &'static F) -> Box<OnMouse<E>>
where
    E: MouseEvent,
    F: Fn(&Element) -> Option<&OnMouse<E>>,
{
    Box::new(move |ctx, e| {
        ctx.layout()
            .get_layout(layout_id)
            .unwrap()
            .defer_mouse_event(ctx, e, f)
    })
}

/// Returns a static `OnMouseUp` getter for convienience
pub fn mouse_up(elem: &Element) -> Option<&OnMouseUp> {
    elem.mouse_up()
}

pub fn mouse_down(elem: &Element) -> Option<&OnMouseDown> {
    elem.mouse_down()
}

/// Returns a static `OnMouseMove` getter for convienience
pub fn mouse_move(elem: &Element) -> Option<&OnMouseMove> {
    elem.mouse_move()
}
