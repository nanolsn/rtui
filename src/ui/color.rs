use crate::{
    common::Color as Col,
    graphics::{
        Draw,
        DrawParameters,
    },
};
use crate::graphics::Render;

#[derive(Debug)]
pub struct Color<U> {
    color: Col,
    ui: U,
}

#[allow(dead_code)]
impl<U> Color<U>
    where
        U: Draw,
{
    pub fn new(ui: U, color: Col) -> Self { Color { ui, color } }

    pub fn black(ui: U) -> Self { Color { ui, color: Col::black() } }
    pub fn white(ui: U) -> Self { Color { ui, color: Col::white() } }
    pub fn red(ui: U) -> Self { Color { ui, color: Col::red() } }
    pub fn green(ui: U) -> Self { Color { ui, color: Col::green() } }
    pub fn blue(ui: U) -> Self { Color { ui, color: Col::blue() } }
}

impl<U> Draw for Color<U>
    where
        U: Draw,
{
    fn draw(&self, render: &mut Render, mut params: DrawParameters) {
        params.color = self.color;
        self.ui.draw(render, params);
    }
}
