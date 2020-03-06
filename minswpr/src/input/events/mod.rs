pub mod backend;

use crate::math::Point;
use crate::{Context, GameState};
use sdl2::mouse::{MouseButton, MouseState};
use std::any::Any;

pub type OnMouse<E> = dyn Fn(&Context, E) -> GameState;
pub type OnMouseUp = OnMouse<MouseUpEvent>;
pub type OnMouseMove = OnMouse<MouseMoveEvent>;
pub type OnMouseDown = OnMouse<MouseDownEvent>;
pub type OnMouseEnter = OnMouse<MouseEnterEvent>;
pub type OnMouseLeave = OnMouse<MouseLeaveEvent>;

/// A generic event that contains a mouse `Point` position
pub trait MouseEvent: AsRef<dyn Any> {
    /// Returns the `Point` position of the mouse
    fn mouse_pos(&self) -> Point;
}

/// Event created when a `MouseButton`, is released on the screen
#[derive(new, AsAny)]
pub struct MouseUpEvent {
    mouse_btn: MouseButton,
    mouse_pos: Point,
}

impl MouseUpEvent {
    /// Returns the `MouseButton` that was released
    pub fn mouse_btn(&self) -> MouseButton {
        self.mouse_btn
    }
}

impl MouseEvent for MouseUpEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

/// Event created when a `MouseButton` is pressed down
#[derive(new, AsAny)]
pub struct MouseDownEvent {
    mouse_btn: MouseButton,
    mouse_pos: Point,
}

impl MouseDownEvent {
    pub fn mouse_btn(&self) -> MouseButton {
        self.mouse_btn
    }
}

impl MouseEvent for MouseDownEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

/// Event thrown when the mouse cursor moves
#[derive(new, AsAny)]
pub struct MouseMoveEvent {
    mouse_state: MouseState,
    mouse_pos: Point,
}

impl MouseMoveEvent {
    pub fn mouse_state(&self) -> MouseState {
        self.mouse_state
    }
}

impl MouseEvent for MouseMoveEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

/// Event thrown when the mouse enters a `layout::Element`
#[derive(new, AsAny)]
pub struct MouseEnterEvent {
    mouse_state: MouseState,
    mouse_pos: Point,
}

impl MouseEnterEvent {
    pub fn mouse_state(&self) -> MouseState {
        self.mouse_state
    }
}

impl MouseEvent for MouseEnterEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

impl From<&MouseMoveEvent> for MouseEnterEvent {
    fn from(e: &MouseMoveEvent) -> Self {
        Self::new(e.mouse_state(), e.mouse_pos())
    }
}

/// Event thrown when the mouse leaves a `layout::Element`
#[derive(new, AsAny)]
pub struct MouseLeaveEvent {
    mouse_state: MouseState,
    mouse_pos: Point,
}

impl MouseLeaveEvent {
    pub fn mouse_state(&self) -> MouseState {
        self.mouse_state
    }
}

impl MouseEvent for MouseLeaveEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

impl From<&MouseMoveEvent> for MouseLeaveEvent {
    fn from(e: &MouseMoveEvent) -> Self {
        Self::new(e.mouse_state(), e.mouse_pos())
    }
}
