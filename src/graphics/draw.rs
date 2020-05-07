use super::{
    super::common::{
        Rect,
        Color,
        Position,
        Vec2d,
    },
    Render,
};

pub struct DrawParameters {
    pub color: Color,
    pub position: Position,
    pub frame: Rect<i32>,
}

impl DrawParameters {
    pub fn render_rect<S>(&self, size: S) -> Rect<i32>
        where
            S: Into<Vec2d<i32>>,
    { self.position.rect(self.frame, size) }
}

pub trait Draw {
    fn draw(&self, render: &mut Render, params: DrawParameters);
}

impl<T> Draw for &T
    where
        T: Draw,
{
    fn draw(&self, render: &mut Render, params: DrawParameters) { (**self).draw(render, params) }
}

impl<T> Draw for &mut T
    where
        T: Draw,
{
    fn draw(&self, render: &mut Render, params: DrawParameters) { (**self).draw(render, params) }
}

impl<T> Draw for Box<T>
    where
        T: Draw,
{
    fn draw(&self, render: &mut Render, params: DrawParameters) { (**self).draw(render, params) }
}

impl<T> Draw for Rect<T>
    where
        T: num::NumCast + Copy,
{
    fn draw(&self, render: &mut Render, params: DrawParameters) {
        render.set_color(params.color);
        render.unset_texture();
        render.draw_rect(self.cast());
    }
}

impl Draw for &str {
    fn draw(&self, render: &mut Render, params: DrawParameters) {
        render.set_color(params.color);
        render.print(self, &params);
    }
}

impl Draw for String {
    fn draw(&self, render: &mut Render, params: DrawParameters) {
        self.as_str().draw(render, params);
    }
}
