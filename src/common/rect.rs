use super::{Pos, Size};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl Rect {
    pub fn new<P, S>(pos: P, size: S) -> Self
        where
            P: Into<Pos>,
            S: Into<Size>,
    {
        let pos = pos.into();
        let size = size.into();

        Rect {
            x: pos.0,
            y: pos.1,
            width: size.0,
            height: size.1,
        }
    }

    pub fn pos(&self) -> Pos { Pos(self.x, self.y) }

    pub fn size(&self) -> Size { Size(self.width, self.height) }

    pub fn left(&self) -> i32 { self.x }

    pub fn right(&self) -> i32 { self.x + self.width as i32 }

    pub fn bot(&self) -> i32 { self.y }

    pub fn top(&self) -> i32 { self.y + self.height as i32 }

    pub fn center(&self) -> Pos { self.pos() + self.size().half() }

    pub fn intersects_point(&self, (a, b): (i32, i32)) -> bool {
        self.left() <= a && a < self.right()
            && self.bot() <= b && b < self.top()
    }

    pub fn intersects_rect(&self, rhs: Rect) -> bool {
        self.left() < rhs.right() && rhs.left() <= self.right()
            && self.bot() < rhs.top() && rhs.bot() <= self.top()
    }
}
