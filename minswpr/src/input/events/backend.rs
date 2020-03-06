use crate::input;
use crate::input::events::{MouseDownEvent, MouseMoveEvent, MouseUpEvent};
use crate::math::Point;
use crate::{Context, GameState};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::EventPump;

pub fn poll_events(ctx: &mut Context, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        ctx.set_game_state(self::handle_event(&ctx, event));
    }
}

fn handle_event(ctx: &Context, event: Event) -> GameState {
    match event {
        Event::Quit { .. } => GameState::Quit,
        Event::MouseButtonUp {
            mouse_btn, x, y, ..
        } => self::handle_mouse_up(ctx, mouse_btn, x, y),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => self::handle_mouse_down(ctx, mouse_btn, x, y),
        Event::MouseMotion {
            mousestate, x, y, ..
        } => self::handle_mouse_motion(ctx, mousestate, x, y),
        Event::KeyDown { keycode, .. } => match keycode {
            Some(k) => self::handle_key_down(ctx, k),
            None => ctx.game_state(),
        },
        _ => ctx.game_state(),
    }
}

fn handle_mouse_up(ctx: &Context, mouse_btn: MouseButton, x: i32, y: i32) -> GameState {
    for button in ctx.buttons() {
        button.borrow_mut().set_released(true);
    }

    ctx.layout().defer_mouse_event(
        ctx,
        MouseUpEvent::new(mouse_btn, point!(x, y)),
        &input::mouse_up,
    )
}

fn handle_mouse_down(ctx: &Context, mouse_btn: MouseButton, x: i32, y: i32) -> GameState {
    ctx.layout().defer_mouse_event(
        ctx,
        MouseDownEvent::new(mouse_btn, point!(x, y)),
        &input::mouse_down,
    )
}

fn handle_mouse_motion(ctx: &Context, mouse_state: MouseState, x: i32, y: i32) -> GameState {
    ctx.layout().defer_mouse_event(
        ctx,
        MouseMoveEvent::new(mouse_state, point!(x, y)),
        &input::mouse_move,
    )
}

fn handle_key_down(ctx: &Context, keycode: Keycode) -> GameState {
    match keycode {
        Keycode::F2 => GameState::Reset,
        _ => ctx.game_state(),
    }
}
