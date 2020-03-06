pub mod control;

use crate::app::context::Context;
use crate::app::GameState;
use crate::draw::{Draw, DrawContext};
use crate::math::{Dimen, Point};
use crate::MsResult;
use sdl2::pixels::Color;
use std::any;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{hash_map, HashMap};
use std::fmt;

use crate::input::events::*;

use self::Orientation::*;

/// Organizes various elements on the canvas
#[derive(Default, Builder, AsAny)]
pub struct Layout {
    #[builder(setter(skip))]
    nodes: HashMap<&'static str, Node>,
    #[builder(setter(skip))]
    hover_id: Cell<&'static str>,
    #[builder(default, setter(strip_option))]
    color: Option<Color>,
    #[builder(default)]
    padding: u32,
    #[builder(default)]
    orientation: Orientation,
    #[builder(default)]
    guides: bool,
}

impl Layout {
    /// Returns the background color of this layout
    pub fn color(&self) -> Option<Color> {
        self.color
    }

    /// Returns the padding (pixels) of this layout
    pub fn padding(&self) -> u32 {
        self.padding
    }

    /// Returns spacial orielementof this layout. That is, the direction in
    /// which elements should be laid-out on the screen
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Inserts a new element into the layout
    ///
    /// # Arguments
    /// * `key` - Unique identified for element
    /// * `order` - The ordinal positioning of the element in this layout
    /// * `elem` - Instance of `Draw` element
    pub fn insert(&mut self, key: &'static str, order: i32, elem: Element) {
        self.nodes.insert(key, Node::new(key, order, elem));
    }

    /// Inserts the elements in the specified `elems` `Vec<_>`
    pub fn insert_all(&mut self, mut elems: Vec<(&'static str, Element)>) {
        for (i, c) in elems.drain(..).enumerate() {
            self.insert(c.0, i as i32, c.1);
        }
    }

    /// Returns `Ok(&Element)` of the element with the specified unique ID,
    /// or `Err(String)` if the element is not present.
    pub fn get(&self, key: &'static str) -> MsResult<&Node> {
        self.nodes
            .get(key)
            .ok_or_else(|| format!("missing required layout element `{}`", key))
    }

    /// Returns `Ok(&Layout)` of the element with the specified unique ID, or
    /// `Err(String)` if the element is not present or if the element found
    /// is not an instance of Layout.
    pub fn get_layout(&self, key: &'static str) -> MsResult<&Layout> {
        self.get(key)?
            .elem()
            .draw_ref()
            .as_ref()
            .downcast_ref::<Layout>()
            .ok_or_else(|| format!("Draw downcast to Layout failed on `{}`", key))
    }

    /// Returns `Some(&Element)` of the element at the specified `x` and `y`
    /// position on the screen. Otherwise, returns None
    pub fn get_at(&self, x: i32, y: i32) -> Option<&Node> {
        for node in self.nodes.values() {
            let Point { x: min_x, y: min_y } = node.pos;
            let cd = node.elem().draw_ref.dimen();
            let max_x = min_x + cd.width() as i32;
            let max_y = min_y + cd.height() as i32;

            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                return Some(node);
            }
        }

        None
    }

    /// Resolves the proper recipient of the specified `MouseEvent` by
    /// retrieving the `Element` at the event's `mouse_pos`. Returns a new
    /// `GameState` as a result.
    ///
    /// # Arguments
    /// * `ctx` - The main `Context`
    /// * `handler_getter` - A function that takes an `&Element` as an argument
    ///   and returns the proper `OnMouse` handler for the event type
    pub fn defer_mouse_event<E, F>(&self, ctx: &Context, e: E, handler_getter: F) -> GameState
    where
        E: MouseEvent,
        F: FnOnce(&Element) -> Option<&OnMouse<E>>,
    {
        // bit of a hack but need to handle MouseMoveEvent for hover tracking
        let game_state = if any::type_name::<E>() == any::type_name::<MouseMoveEvent>() {
            self.on_mouse_move(ctx, e.as_ref().downcast_ref::<MouseMoveEvent>().unwrap())
        } else {
            ctx.game_state()
        };

        let Point { x, y } = e.mouse_pos();
        match self.get_at(x, y) {
            Some(n) => match handler_getter(n.elem()) {
                Some(handler) => handler(ctx, e),
                None => game_state,
            },
            None => game_state,
        }
    }

    pub fn on_mouse_move(&self, ctx: &Context, e: &MouseMoveEvent) -> GameState {
        let pos = e.mouse_pos();
        let hover_id = self.hover_id.get();
        match self.get_at(pos.x, pos.y) {
            Some(n) => {
                if hover_id == n.id() {
                    ctx.game_state()
                } else {
                    self.on_hover_change(ctx, e, n)
                }
            }
            None => {
                let game_state = match hover_id {
                    "" => ctx.game_state(),
                    _ => self.fire_mouse_leave_event(ctx, e),
                };
                self.hover_id.set("");
                game_state
            }
        }
    }

    fn on_hover_change(&self, ctx: &Context, e: &MouseMoveEvent, node: &Node) -> GameState {
        let game_state = self.fire_mouse_leave_event(ctx, e);
        let game_state = match node.elem().mouse_enter() {
            Some(enter) => enter(ctx, MouseEnterEvent::from(e)),
            None => game_state,
        };
        self.hover_id.set(node.id());
        game_state
    }

    fn fire_mouse_leave_event(&self, ctx: &Context, e: &MouseMoveEvent) -> GameState {
        match self.get(self.hover_id.get()) {
            Ok(old) => match old.elem().mouse_leave() {
                Some(leave) => leave(ctx, MouseLeaveEvent::from(e)),
                None => ctx.game_state(),
            },
            Err(_) => ctx.game_state(),
        }
    }
}

impl Draw for Layout {
    fn draw(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        if let Some(c) = self.color {
            draw_rect!(self.dimen(), c, ctx, pos)?;
        }

        let orien = self.orientation;
        let padding = self.padding;
        let mut cur = pos + point!(padding, padding).as_i32();
        let mut nodes = self.nodes.values_mut().collect::<Vec<_>>();

        nodes.sort();

        for node in nodes {
            let r = &mut node.elem.draw_ref;
            let m = r.margins();

            node.pos = cur + point!(m.left, m.top).as_i32();
            r.draw(ctx, node.pos)?;

            let d = r.dimen();

            cur += match &orien {
                Vertical => point!(0, d.height() + m.bottom + m.top).as_i32(),
                Horizontal => point!(d.width() + m.right + m.left, 0).as_i32(),
            };
        }

        if self.guides {
            self.draw_guides(ctx, pos)
        } else {
            Ok(())
        }
    }

    fn dimen(&self) -> Dimen {
        let (d, acc): (Dimen, Box<dyn Fn(Dimen, Dimen) -> Dimen>) = match self.orientation {
            Vertical => (
                point!(self.node_max_width(), 0),
                Box::new(|a, b| a + (0, b.height())),
            ),
            Horizontal => (
                point!(0, self.node_max_height()),
                Box::new(|a, b| a + (b.width(), 0)),
            ),
        };

        self.calc_dimen(d, acc)
    }
}

type NodeValues<'a> = hash_map::Values<'a, &'static str, Node>;

impl Layout {
    fn draw_guides(&mut self, ctx: &DrawContext, pos: Point) -> MsResult {
        // for debugging
        let Dimen { x: w, y: h } = self.dimen();
        draw_rect!(
            point!(1, h),
            color!(magenta),
            ctx,
            pos + point!(w / 2, 0).as_i32()
        )?;
        draw_rect!(
            point!(w, 1),
            color!(magenta),
            ctx,
            pos + point!(0, h / 2).as_i32()
        )
    }

    fn node_values(&self) -> NodeValues {
        self.nodes.values()
    }

    fn node_max_height(&self) -> u32 {
        self.node_values()
            .map(|c| c.elem().draw_ref.dimen().height())
            .max()
            .unwrap_or_else(|| 0)
    }

    fn node_max_width(&self) -> u32 {
        self.node_values()
            .map(|c| c.elem().draw_ref.dimen().width())
            .max()
            .unwrap_or_else(|| 0)
    }

    fn calc_dimen(&self, initial: Dimen, acc: impl Fn(Dimen, Dimen) -> Dimen) -> Dimen {
        let margins: Dimen = self
            .node_values()
            .map(|c| c.elem().draw_ref.margins())
            .map(|m| point!(m.left + m.right, m.top + m.bottom))
            .sum();

        self.node_values()
            .map(|c| c.elem().draw_ref.dimen())
            .fold(initial, |a, b| acc(a, b))
            + (self.padding * 2, self.padding * 2)
            + margins
    }
}

/// A 2-dimensional orientation in space
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Vertical
    }
}

/// An element on a `Layout`, contained within a `Node`
#[derive(new, Builder)]
#[builder(pattern = "owned")]
pub struct Element {
    draw_ref: Box<dyn Draw>,
    #[builder(default, setter(strip_option))]
    #[new(default)]
    mouse_up: Option<Box<OnMouseUp>>,
    #[builder(default, setter(strip_option))]
    #[new(default)]
    mouse_move: Option<Box<OnMouseMove>>,
    #[builder(default, setter(strip_option))]
    #[new(default)]
    mouse_down: Option<Box<OnMouseDown>>,
    #[builder(default, setter(strip_option))]
    #[new(default)]
    mouse_enter: Option<Box<OnMouseEnter>>,
    #[builder(default, setter(strip_option))]
    #[new(default)]
    mouse_leave: Option<Box<OnMouseLeave>>,
}

impl Element {
    /// Returns a reference to the `Draw` instance contained within this
    /// element
    pub fn draw_ref(&self) -> &dyn Draw {
        &*self.draw_ref
    }

    /// Returns `Some` reference to the `OnMouseUp` handler if present, returns
    /// `None` otherwise
    pub fn mouse_up(&self) -> Option<&OnMouseUp> {
        self.mouse_up.as_deref()
    }

    /// Returns `Some` reference to the `OnMouseMove` handler if present,
    /// returns `None` otherwise
    pub fn mouse_move(&self) -> Option<&OnMouseMove> {
        self.mouse_move.as_deref()
    }

    /// Returns `Some` reference to the `OnMouseDown` handler if present,
    /// returns `None` otherwise
    pub fn mouse_down(&self) -> Option<&OnMouseDown> {
        self.mouse_down.as_deref()
    }

    /// Returns `Some` reference to the `OnMouseEnter` handler if present,
    /// returns `None` otherwise
    pub fn mouse_enter(&self) -> Option<&OnMouseEnter> {
        self.mouse_enter.as_deref()
    }

    /// Returns `Some` reference to the `OnMouseLeave` handler if present,
    /// returns `None` otherwise
    pub fn mouse_leave(&self) -> Option<&OnMouseLeave> {
        self.mouse_leave.as_deref()
    }
}

/// A single node within the layout
#[derive(new)]
pub struct Node {
    id: &'static str,
    order: i32,
    #[new(default)]
    pos: Point,
    elem: Element,
}

impl Node {
    /// Returns this nodes unique identifier
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// Returns the current position of this node
    pub fn pos(&self) -> Point {
        self.pos
    }

    /// Returns the contains `Element` in this node
    pub fn elem(&self) -> &Element {
        &self.elem
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for Node {}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node {{ order: {}, pos: {:?}, dimen: {:?} }}",
            self.order,
            self.pos,
            self.elem().draw_ref.dimen()
        )
    }
}
