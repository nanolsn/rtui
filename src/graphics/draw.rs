use super::{
    super::common::{
        Rect,
        Color,
    },
    Render,
};

pub struct DrawParameters {
    pub color: Color,
}

impl Default for DrawParameters {
    fn default() -> Self {
        DrawParameters {
            color: Color::white(),
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
        let rect: Rect<f32> = self.cast();

        render.set_color(params.color);
        render.unset_texture();
        render.draw_rect(rect);
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
