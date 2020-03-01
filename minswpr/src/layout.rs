use crate::math::{Dimen, Point};
use crate::render::{Render, RenderMut};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::cmp::Ordering;
use std::collections::HashMap;

pub type RenderRef<'a> = Box<dyn Render + 'a>;

#[derive(new)]
pub struct Layout<'a> {
    #[new(default)]
    components: HashMap<&'static str, Component<'a>>,
    padding: u32,
    color: Color,
}

impl<'a> Layout<'a> {
    pub fn insert(&mut self, key: &'static str, order: i32, component: RenderRef<'a>) {
        self.components
            .insert(key, Component::new(order, component));
    }

    pub fn insert_all(&mut self, mut components: Vec<(&'static str, Option<RenderRef<'a>>)>) {
        for (i, c) in components.iter_mut().enumerate() {
            self.insert(c.0, i as i32, c.1.take().unwrap());
        }
    }

    pub fn get(&self, key: &'static str) -> Result<&Component, String> {
        self.components
            .get(key)
            .ok_or_else(|| format!("missing required layout component `{}`", key))
    }
}

impl RenderMut for Layout<'_> {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        render_rect!(self.dimen(), self.color, canvas, pos)?;

        let mut cur = pos + point!(self.padding, self.padding).as_i32();

        let components = &mut self
            .components
            .values_mut()
            .collect::<Vec<&mut Component>>();

        components.sort();

        for component in components {
            component.pos = cur;
            let r = &component.render;
            r.render(canvas, cur)?;
            cur += (0, r.dimen().height() as i32);
        }

        Ok(())
    }

    fn dimen(&self) -> Dimen {
        let values = || self.components.values();
        let width = values().map(|c| c.render.dimen().width()).max().unwrap();
        values().fold(point!(width, 0), |a, b| a + (0, b.render.dimen().height()))
            + (self.padding * 2, self.padding * 2)
    }
}

#[derive(new)]
pub struct Component<'a> {
    order: i32,
    render: Box<dyn Render + 'a>,
    #[new(default)]
    pos: Point,
}

impl<'a> Component<'a> {
    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn render(&self) -> &(dyn Render + 'a) {
        &*self.render
    }
}

impl PartialOrd for Component<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Component<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialEq for Component<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for Component<'_> {}
