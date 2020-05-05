use crate::{
    common::Position,
    graphics::{
        Render,
        Draw,
        DrawParameters,
    },
};

#[derive(Debug)]
pub struct Pos<U> {
    position: Position,
    ui: U,
}

#[allow(dead_code)]
impl<U> Pos<U>
    where
        U: Draw,
{
    pub fn new(ui: U, position: Position) -> Self { Pos { ui, position } }

    pub fn center(ui: U) -> Self { Pos::new(ui, Position::Center) }
    pub fn left(ui: U, pad: i32) -> Self { Pos::new(ui, Position::Left(pad)) }
    pub fn right(ui: U, pad: i32) -> Self { Pos::new(ui, Position::Right(pad)) }
    pub fn bot(ui: U, pad: i32) -> Self { Pos::new(ui, Position::Bot(pad)) }
    pub fn top(ui: U, pad: i32) -> Self { Pos::new(ui, Position::Top(pad)) }
}

impl<U> Draw for Pos<U>
    where
        U: Draw,
{
    fn draw(&self, render: &mut Render, mut params: DrawParameters) {
        params.position = self.position;
        self.ui.draw(render, params);
    }
}

impl<U> std::ops::Deref for Pos<U> {
    type Target = U;

    fn deref(&self) -> &Self::Target { &self.ui }
}

impl<U> std::ops::DerefMut for Pos<U> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.ui }
}
