use super::{
    Rect,
    Vec2d,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {
    Center,
    Left(i32),
    Right(i32),
    Bot(i32),
    Top(i32),
    LeftBot(i32, i32),
    LeftTop(i32, i32),
    RightBot(i32, i32),
    RightTop(i32, i32),
}

impl Position {
    pub fn rect<S>(self, frame: Rect<i32>, size: S) -> Rect<i32>
        where
            S: Into<Vec2d<i32>>,
    {
        let size = size.into();

        let pos = match self {
            Position::Center => frame.size().half() - size.half(),
            Position::Left(pad) => Vec2d::new(pad, frame.height / 2 - size.y / 2),
            Position::Right(pad) => Vec2d::new(
                frame.width - size.x - pad,
                frame.height / 2 - size.y / 2,
            ),
            Position::Bot(pad) => Vec2d::new(frame.width / 2 - size.x / 2, pad),
            Position::Top(pad) => Vec2d::new(
                frame.width / 2 - size.x / 2,
                frame.height - size.y - pad,
            ),
            Position::LeftBot(l, b) => Vec2d::new(l, b),
            Position::LeftTop(l, t) => Vec2d::new(l, frame.height - size.y - t),
            Position::RightBot(r, b) => Vec2d::new(frame.width - size.x - r, b),
            Position::RightTop(r, t) => Vec2d::new(
                frame.width - size.x - r,
                frame.height - size.y - t,
            ),
        };

        Rect::new(pos, size)
    }
}

impl Default for Position {
    fn default() -> Self { Position::Center }
}
