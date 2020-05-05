use super::{
    super::common::{
        Rect,
        Color,
        Position,
    },
    Render,
};

pub struct DrawParameters {
    pub color: Color,
    pub position: Position,
}

impl Default for DrawParameters {
    fn default() -> Self {
        DrawParameters {
            color: Color::white(),
            position: Position::default(),
        }
    }
}

pub trait Draw {
    fn draw(&self, render: &mut Render, params: DrawParameters);
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
        render.print(self);
    }
}

impl Draw for String {
    fn draw(&self, render: &mut Render, params: DrawParameters) {
        self.as_str().draw(render, params);
    }
}
