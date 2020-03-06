use super::events::{MouseDownEvent, MouseUpEvent};
use crate::{Context, GameState};

pub fn on_reset_mouse_down(ctx: &Context, _: MouseDownEvent) -> GameState {
    ctx.reset_button().borrow_mut().set_pressed(true);
    ctx.game_state()
}

pub fn on_reset_mouse_up(ctx: &Context, _: MouseUpEvent) -> GameState {
    ctx.reset_button().borrow_mut().set_pressed(false);
    GameState::Reset
}
