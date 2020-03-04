#[macro_use]
mod macros;

pub mod board;
pub mod control;
pub mod text;

use crate::fonts::Fonts;
use crate::math::{Dimen, Point};
use crate::MsResult;
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

pub trait Draw: AsRef<dyn Any> {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult;

    fn dimen(&self) -> Dimen;

    fn margins(&self) -> Margins {
        Default::default()
    }
}

#[derive(new)]
pub struct DrawContext<'a> {
    canvas: CanvasRef,
    fonts: &'a Fonts<'a>,
    textures: Textures,
}

impl DrawContext<'_> {
    pub fn canvas(&self) -> CanvasRefMut {
        self.canvas.borrow_mut()
    }

    pub fn with_canvas(&self, f: impl FnOnce(CanvasRefMut)) {
        f(self.canvas.borrow_mut())
    }

    pub fn fonts(&self) -> &Fonts {
        self.fonts
    }

    pub fn textures(&self) -> &Textures {
        &self.textures
    }
}

#[derive(new, AsAny)]
pub struct DrawRect {
    dimen: Dimen,
    color: Color,
    #[new(default)]
    margins: Margins,
}

impl DrawRect {
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
    pub fn top(&mut self, top: u32) -> &mut Self {
        self.top = top;
        self
    }

    pub fn right(&mut self, right: u32) -> &mut Self {
        self.right = right;
        self
    }

    pub fn bottom(&mut self, bottom: u32) -> &mut Self {
        self.bottom = bottom;
        self
    }

    pub fn left(&mut self, left: u32) -> &mut Self {
        self.left = left;
        self
    }
}
