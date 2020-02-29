use crate::math::{Dimen, Point};
use crate::render::Render;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;

#[derive(new)]
pub struct Layout<'a> {
    #[new(default)]
    components: HashMap<&'static str, Component<'a>>,
}

impl<'a> Layout<'a> {
    pub fn insert(&mut self, key: &'static str, order: i32, component: Box<dyn Render + 'a>) {
        self.components
            .insert(key, Component::new(order, component));
    }

    pub fn get(&self, key: &'static str) -> Result<&dyn Render, String> {
        Ok(self
            .components
            .get(key)
            .ok_or_else(|| format!("missing required layout component `{}`", key))?
            .render
            .as_ref())
    }
}

impl<'a> Render for Layout<'a> {
    fn render(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String> {
        let mut cur = *pos;
        for component in self.components.values() {
            let r = &component.render;
            r.render(canvas, &cur)?;
            cur += r.dimen().as_i32();
        }
        Ok(())
    }

    fn dimen(&self) -> Dimen {
        // TODO
        point!(0, 0)
    }
}

#[derive(new)]
struct Component<'a> {
    _order: i32, // TODO
    render: Box<dyn Render + 'a>,
}
