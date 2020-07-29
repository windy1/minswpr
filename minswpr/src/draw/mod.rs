#[macro_use]
mod macros;

/// Implements `Draw` for the main `Board`
pub mod board;
/// Implements the components required to draw the control panel located above
/// the board
pub mod control;
/// Implements text rendering
pub mod text;

use crate::{GameState, MsResult};
use reba::fonts::Fonts;
use reba::math::{Dimen, Point};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub type CanvasRefMut<'a> = RefMut<'a, WindowCanvas>;
pub type CanvasRef = Rc<RefCell<WindowCanvas>>;
pub type Textures = TextureCreator<WindowContext>;

/// Something that can be drawn to a `sdl2::render::WindowCanvas`
pub trait Draw: AsRef<dyn Any> {
    /// Draws this to the canvas using the specified `DrawContext` at the
    /// specied `Point` on the screen
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult;

    /// Returns the `Dimen` for this
    fn dimen(&self) -> Dimen;

    /// Returns the `Margins` for this
    fn margins(&self) -> Margins {
        Default::default()
    }
}

/// Contains the necessary components to draw to the canvas
#[derive(new)]
pub struct DrawContext<'a> {
    canvas: CanvasRef,
    fonts: &'a Fonts<'a>,
    textures: Textures,
    game_state: GameState,
}

impl DrawContext<'_> {
    /// Returns a mutable reference to the canvas
    pub fn canvas(&self) -> CanvasRefMut {
        self.canvas.borrow_mut()
    }

    /// Calls the specified function with a mutable reference to the canvas as
    /// an argument
    ///
    /// # Arguments
    /// * `f` - The function that will use the borrowed canvas
    pub fn with_canvas(&self, f: impl FnOnce(CanvasRefMut)) {
        f(self.canvas.borrow_mut())
    }

    /// Returns a reference to the `Fonts` instance
    pub fn fonts(&self) -> &Fonts {
        self.fonts
    }

    /// Returns a reference to the `TextureCreator`
    pub fn textures(&self) -> &Textures {
        &self.textures
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    pub fn set_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state
    }
}

/// Helper struct for drawing a basic rectangle to the canvas
#[derive(new, AsAny)]
pub struct DrawRect {
    dimen: Dimen,
    color: Color,
    #[new(default)]
    margins: Margins,
}

impl DrawRect {
    /// Creates a new `DrawRect`
    ///
    /// # Arguments
    /// * `dimen` - The dimensions of the rectangle
    /// * `color` - The rectangle Color
    /// * `margins` - The `Margins` of the rectangle for spacing relative to
    ///   other elements
    pub fn with_margins(dimen: Dimen, color: Color, margins: Margins) -> Self {
        Self {
            dimen,
            color,
            margins,
        }
    }
}

impl Draw for DrawRect {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        let mut canvas = ctx.canvas();
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(
            pos.x,
            pos.y,
            self.dimen.width(),
            self.dimen.height(),
        ))
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }

    fn margins(&self) -> Margins {
        self.margins
    }
}

/// Contains margin data for `Draw` elements in a `Layout`. The `Margins` of a
/// `Draw` determines how the element is spaced relative to other elements
/// in the layout.
#[derive(new, Default, Clone, Copy, Debug)]
pub struct Margins {
    #[new(default)]
    pub top: u32,
    #[new(default)]
    pub right: u32,
    #[new(default)]
    pub bottom: u32,
    #[new(default)]
    pub left: u32,
}

impl Margins {
    /// Sets the `top` margin
    pub fn top(&mut self, top: u32) -> &mut Self {
        self.top = top;
        self
    }

    /// Sets the `right` margin
    pub fn right(&mut self, right: u32) -> &mut Self {
        self.right = right;
        self
    }

    /// Sets the `bottom` margin
    pub fn bottom(&mut self, bottom: u32) -> &mut Self {
        self.bottom = bottom;
        self
    }

    /// Sets the `left` margin
    pub fn left(&mut self, left: u32) -> &mut Self {
        self.left = left;
        self
    }
}
