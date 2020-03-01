use super::input::{Execute, KeyDown, MouseUp};
use super::math::Point;
use super::{Context, GameState};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub fn handle_event(context: &Context, event: Event) -> Result<GameState, String> {
    match event {
        Event::Quit { .. } => Ok(GameState::Quit),
        Event::MouseButtonUp {
            mouse_btn, x, y, ..
        } => self::handle_mouse_up(context, mouse_btn, x, y),
        Event::KeyDown { keycode, .. } => match keycode {
            Some(k) => self::handle_key_down(context, k),
            None => Ok(*context.game_state()),
        },
        _ => Ok(*context.game_state()),
    }
}

fn handle_mouse_up(
    context: &Context,
    mouse_btn: MouseButton,
    x: i32,
    y: i32,
) -> Result<GameState, String> {
    MouseUp::new(mouse_btn, point!(x, y), context).execute()
}

fn handle_key_down(context: &Context, keycode: Keycode) -> Result<GameState, String> {
    KeyDown::new(keycode, context).execute()
}
