use crate::{
    common::Color,
    graphics::{
        Render,
        Draw,
        DrawParameters,
    },
};

#[derive(Debug)]
pub struct Col<U> {
    color: Color,
    ui: U,
}

#[allow(dead_code)]
impl<U> Col<U>
    where
        U: Draw,
{
    pub fn new(color: Color, ui: U) -> Self { Col { color, ui } }

    pub fn black(ui: U) -> Self { Col::new(Color::black(), ui) }
    pub fn white(ui: U) -> Self { Col::new(Color::white(), ui) }
    pub fn red(ui: U) -> Self { Col::new(Color::red(), ui) }
    pub fn green(ui: U) -> Self { Col::new(Color::green(), ui) }
    pub fn blue(ui: U) -> Self { Col::new(Color::blue(), ui) }
}

impl<U> Draw for Col<U>
    where
        U: Draw,
{
    fn draw(&self, render: &mut Render, mut params: DrawParameters) {
        params.color = self.color;
        self.ui.draw(render, params);
    }
}

impl<U> std::ops::Deref for Col<U> {
    type Target = U;

    fn deref(&self) -> &Self::Target { &self.ui }
}

impl<U> std::ops::DerefMut for Col<U> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.ui }
}
