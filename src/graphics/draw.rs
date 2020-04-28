use super::Render;

pub trait Draw {
    fn draw(&self, render: &mut Render);
}

impl Draw for crate::common::Rect {
    fn draw(&self, render: &mut Render) { render.draw_rect(*self) }
}
