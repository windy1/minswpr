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

    let mut btn = ctx.buttons()["reset"].borrow_mut();
    btn.set_pressed(true);
    btn.set_released(false);

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

    let mut btn = ctx.buttons()["reset"].borrow_mut();
    btn.set_pressed(false);
    btn.set_released(true);

    GameState::Reset
}

pub fn on_mouse_leave_reset_button(ctx: &Context, _: MouseLeaveEvent) -> GameState {
    ctx.buttons()["reset"].borrow_mut().set_pressed(false);
    ctx.game_state()
}

pub fn on_mouse_enter_reset_button(ctx: &Context, e: MouseEnterEvent) -> GameState {
    let mut btn = ctx.buttons()["reset"].borrow_mut();
    if e.mouse_state().is_mouse_button_pressed(MouseButton::Left) && !btn.is_released() {
        btn.set_pressed(true);
    }
    ctx.game_state()
}
