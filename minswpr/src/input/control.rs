use super::events::{MouseDownEvent, MouseEnterEvent, MouseLeaveEvent, MouseUpEvent};
use crate::{Context, GameState};
use sdl2::mouse::MouseButton;

pub fn on_mouse_down_reset_button(ctx: &Context, e: MouseDownEvent) -> GameState {
    match e.mouse_btn() {
        MouseButton::Left => {}
        MouseButton::Middle => {
            // TODO
            return ctx.game_state();
        }
        _ => return ctx.game_state(),
    }
    ctx.reset_button().borrow_mut().set_pressed(true);
    ctx.game_state()
}

pub fn on_mouse_up_reset_button(ctx: &Context, e: MouseUpEvent) -> GameState {
    match e.mouse_btn() {
        MouseButton::Left => {}
        MouseButton::Middle => {
            // TODO
            return ctx.game_state();
        }
        _ => return ctx.game_state(),
    }
    ctx.reset_button().borrow_mut().set_pressed(false);
    GameState::Reset
}

pub fn on_mouse_leave_reset_button(ctx: &Context, e: MouseLeaveEvent) -> GameState {
    ctx.reset_button().borrow_mut().set_pressed(false);
    ctx.game_state()
}

pub fn on_mouse_enter_reset_button(ctx: &Context, e: MouseEnterEvent) -> GameState {
    ctx.game_state()
}
