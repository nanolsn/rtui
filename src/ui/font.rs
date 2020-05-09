use crate::{
    common::{
        Vec2d,
        Color,
    },
    graphics::{
        Render,
        Draw,
        DrawParameters,
        FontParameters,
    },
};

#[derive(Debug)]
pub struct Font<U> {
    font: FontParameters,
    ui: U,
}

impl<U> Font<U>
    where
        U: Draw,
{
    pub fn new(ui: U) -> Self {
        Font::from_style(ui, FontParameters {
            monospaced: false,
            shadow: None,
        })
    }

    pub fn from_style(ui: U, font: FontParameters) -> Self { Font { ui, font } }

    pub fn monospaced(mut self) -> Self {
        self.font = self.font.monospaced();
        self
    }

    pub fn shadow<V>(mut self, delta: V, color: Color) -> Self
        where
            V: Into<Vec2d<i32>>,
    {
        self.font = self.font.shadow(delta, color);
        self
    }
}

impl<U> Draw for Font<U>
    where
        U: Draw,
{
    fn draw(&self, render: &mut Render, mut params: DrawParameters) {
        params.font = self.font;
        self.ui.draw(render, params);
    }
}
