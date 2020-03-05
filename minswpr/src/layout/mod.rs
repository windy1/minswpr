use crate::draw::{Draw, DrawContext};
use crate::math::{Dimen, Point};
use crate::MsResult;
use sdl2::pixels::Color;
use std::cmp::Ordering;
use std::collections::{hash_map, HashMap};
use std::fmt;

use self::Orientation::*;

/// Organizes various components on the canvas
#[derive(Builder, AsAny)]
pub struct Layout {
    #[builder(setter(skip))]
    components: HashMap<&'static str, Component>,
    #[builder(default)]
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

    /// Returns spacial orientation of this layout. That is, the direction in
    /// which components should be laid-out on the screen
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Inserts a new component into the layout
    ///
    /// # Arguments
    /// * `key` - Unique identified for component
    /// * `order` - The ordinal positioning of the component in this layout
    /// * `component` - Instance of `Draw` component
    pub fn insert(&mut self, key: &'static str, order: i32, component: Box<dyn Draw>) {
        self.components
            .insert(key, Component::new(key, order, component));
    }

    /// Inserts the components in the specified `components` `Vec<_>`
    pub fn insert_all(&mut self, mut components: Vec<(&'static str, Box<dyn Draw>)>) {
        for (i, c) in components.drain(..).enumerate() {
            self.insert(c.0, i as i32, c.1);
        }
    }

    /// Returns `Ok(&Component)` of the component with the specified unique ID,
    /// or `Err(String)` if the component is not present.
    pub fn get(&self, key: &'static str) -> MsResult<&Component> {
        self.components
            .get(key)
            .ok_or_else(|| format!("missing required layout component `{}`", key))
    }

    /// Returns `Ok(&Layout)` of the component with the specified unique ID, or
    /// `Err(String)` if the component is not present or if the component found
    /// is not an instance of Layout.
    pub fn get_layout(&self, key: &'static str) -> MsResult<&Layout> {
        self.get(key)?
            .draw_ref()
            .as_ref()
            .downcast_ref::<Layout>()
            .ok_or_else(|| format!("Draw downcast to Layout failed on `{}`", key))
    }

    /// Returns `Some(&Component)` of the component at the specified `x` and `y`
    /// position on the screen. Otherwise, returns None
    pub fn get_at(&self, x: i32, y: i32) -> Option<&Component> {
        for component in self.components.values() {
            let Point { x: min_x, y: min_y } = component.pos;
            let cd = component.draw_ref.dimen();
            let max_x = min_x + cd.width() as i32;
            let max_y = min_y + cd.height() as i32;

            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                return Some(component);
            }
        }

        None
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
        let mut components = self.components.values_mut().collect::<Vec<_>>();

        components.sort();

        for component in components {
            let r = &mut component.draw_ref;
            let m = r.margins();

            component.pos = cur + point!(m.left, m.top).as_i32();
            r.draw(ctx, component.pos)?;

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
                point!(self.component_max_width(), 0),
                Box::new(|a, b| a + (0, b.height())),
            ),
            Horizontal => (
                point!(0, self.component_max_height()),
                Box::new(|a, b| a + (b.width(), 0)),
            ),
        };

        self.calc_dimen(d, acc)
    }
}

type ComponentValues<'a> = hash_map::Values<'a, &'static str, Component>;

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

    fn component_values(&self) -> ComponentValues {
        self.components.values()
    }

    fn component_max_height(&self) -> u32 {
        self.component_values()
            .map(|c| c.draw_ref.dimen().height())
            .max()
            .unwrap_or_else(|| 0)
    }

    fn component_max_width(&self) -> u32 {
        self.component_values()
            .map(|c| c.draw_ref.dimen().width())
            .max()
            .unwrap_or_else(|| 0)
    }

    fn calc_dimen(&self, initial: Dimen, acc: impl Fn(Dimen, Dimen) -> Dimen) -> Dimen {
        let margins: Dimen = self
            .component_values()
            .map(|c| c.draw_ref.margins())
            .map(|m| point!(m.left + m.right, m.top + m.bottom))
            .sum();

        self.component_values()
            .map(|c| c.draw_ref.dimen())
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

/// A single component within the layout
#[derive(new)]
pub struct Component {
    id: &'static str,
    order: i32,
    draw_ref: Box<dyn Draw>,
    #[new(default)]
    pos: Point,
}

impl Component {
    /// Returns this components unique identifier
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// Returns the current position of this component
    pub fn pos(&self) -> Point {
        self.pos
    }

    /// Returns a reference to the `Draw` instance contained within this
    /// compoent
    pub fn draw_ref(&self) -> &dyn Draw {
        &*self.draw_ref
    }
}

impl PartialOrd for Component {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Component {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for Component {}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Component {{ order: {}, pos: {:?}, dimen: {:?} }}",
            self.order,
            self.pos,
            self.draw_ref.dimen()
        )
    }
}
