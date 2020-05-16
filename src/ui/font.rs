use crate::{
    common::{
        Vec2d,
        Color,
    },
    graphics::{
        Render,
        Draw,
        DrawParameters,
        FontStyle,
    },
};

#[derive(Debug)]
pub struct Font<U> {
    style: FontStyle,
    ui: U,
}

impl<U> Font<U>
    where
        U: Draw,
{
    pub fn new(ui: U) -> Self {
        Font::from_style(ui, FontStyle {
            monospaced: false,
            shadow: None,
        })
    }

    pub fn from_style(ui: U, font: FontStyle) -> Self { Font { ui, style: font } }

    #[allow(dead_code)]
    pub fn monospaced(mut self) -> Self {
        self.style = self.style.monospaced();
        self
    }

    pub fn shadow<V>(mut self, delta: V, color: Color) -> Self
        where
            V: Into<Vec2d<i32>>,
    {
        self.style = self.style.shadow(delta, color);
        self
    }
}

impl<U> Draw for Font<U>
    where
        U: Draw,
{
    fn draw(&self, render: &mut Render, mut params: DrawParameters) {
        params.font_style = self.style;
        self.ui.draw(render, params);
    }
}
