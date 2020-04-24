use std::ops::{Mul, Div};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new((x, y): (u32, u32), (width, height): (u32, u32)) -> Self {
        Rect { x, y, width, height }
    }

    pub fn pos(&self) -> (u32, u32) { (self.x, self.y) }

    pub fn size(&self) -> (u32, u32) { (self.width, self.height) }

    pub fn left(&self) -> u32 { self.x }

    pub fn right(&self) -> u32 { self.x + self.width }

    pub fn bot(&self) -> u32 { self.y }

    pub fn top(&self) -> u32 { self.y + self.height }

    pub fn intersects_point(&self, (a, b): (u32, u32)) -> bool {
        self.left() <= a && a < self.right()
            && self.bot() <= b && b < self.top()
    }

    pub fn intersects_rect(&self, rhs: Rect) -> bool {
        self.left() < rhs.right() && rhs.left() <= self.right()
            && self.bot() < rhs.top() && rhs.bot() <= self.top()
    }
}
