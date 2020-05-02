use super::{
    super::Rect,
    Render,
    UsedShader,
};

pub trait Draw {
    fn draw(&self, render: &mut Render);
}

impl<T> Draw for Rect<T>
    where
        T: num::NumCast + Copy,
{
    fn draw(&self, render: &mut Render) {
        let rect: Rect<f32> = self.cast();

        render.unset_texture();
        render.draw_rect(UsedShader::Base, rect);
    }
}

impl Draw for &str {
    fn draw(&self, render: &mut Render) { render.print(self) }
}

impl Draw for String {
    fn draw(&self, render: &mut Render) { self.as_str().draw(render) }
}
