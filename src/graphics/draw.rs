use super::{
    super::Rect,
    Render,
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
        render.draw_rect(rect);
    }
}
