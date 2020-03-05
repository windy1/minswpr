use crate::math::Point;
use crate::{Context, GameState};
use sdl2::mouse::{MouseButton, MouseState};

pub type OnMouse<E> = dyn Fn(&Context, E) -> GameState;
pub type OnMouseUp = OnMouse<MouseUpEvent>;
pub type OnMouseMove = OnMouse<MouseMoveEvent>;
pub type OnMouseDown = OnMouse<MouseDownEvent>;
pub type OnMouseEnter = OnMouse<MouseEnterEvent>;
pub type OnMouseLeave = OnMouse<MouseLeaveEvent>;

/// A generic event that contains a mouse `Point` position
pub trait MouseEvent {
    /// Returns the `Point` position of the mouse
    fn mouse_pos(&self) -> Point;
}

/// Event created when a `MouseButton`, is released on the screen
#[derive(new)]
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
#[derive(new)]
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
#[derive(new)]
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
#[derive(new)]
pub struct MouseEnterEvent {
    mouse_pos: Point,
}

impl MouseEvent for MouseEnterEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}

/// Event thrown when the mouse leaves a `layout::Element`
#[derive(new)]
pub struct MouseLeaveEvent {
    mouse_pos: Point,
}

impl MouseEvent for MouseLeaveEvent {
    fn mouse_pos(&self) -> Point {
        self.mouse_pos
    }
}