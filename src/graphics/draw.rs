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

        let pos = params.position.rect(
            render.size().into_rect(),
            render.font().text_size(self),
        ).pos();

        render.print(self, pos);
    }
}

impl Draw for String {
    fn draw(&self, render: &mut Render, params: DrawParameters) {
        self.as_str().draw(render, params);
    }
}
