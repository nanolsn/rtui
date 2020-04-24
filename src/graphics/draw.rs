use super::Render;

pub trait Draw {
    fn draw(&self, render: &Render);
}

impl Draw for crate::common::Rect {
    fn draw(&self, render: &Render) { render.draw_rect(self) }
}
