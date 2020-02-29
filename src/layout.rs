use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;

pub struct Layout<'a> {
    components: HashMap<&'static str, Box<dyn Render + 'a>>,
}

impl<'a> Layout<'a> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &'static str, component: Box<dyn Render + 'a>) {
        self.components.insert(key, component);
    }

    pub fn get(&self, key: &'static str) -> Result<&dyn Render, String> {
        Ok(self
            .components
            .get(key)
            .ok_or_else(|| format!("missing required layout component `{}`", key))?
            .as_ref())
    }
}

impl<'a> Render for Layout<'a> {
    fn render(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        let mut cur = *pos;
        for component in self.components.values() {
            component.render(canvas, &cur)?;
            cur += component.dimen().as_i32();
        }
        Ok(())
    }

    fn dimen(&self) -> Dimen {
        // TODO
        point!(0, 0)
    }
}
