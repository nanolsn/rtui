use super::{
    Rect,
    Vec2D,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {
    Center,
    Left(i32),
    Right(i32),
    Bot(i32),
    Top(i32),
}

impl Position {
    pub fn rect<S>(self, frame: Rect<i32>, size: S) -> Rect<i32>
        where
            S: Into<Vec2D<i32>>,
    {
        let size = size.into();

        let pos = match self {
            Position::Center => frame.size().half() - size.half(),
            Position::Left(pad) => Vec2D::new(pad, frame.height / 2 - size.y / 2),
            Position::Right(pad) => Vec2D::new(
                frame.width - size.x - pad,
                frame.height / 2 - size.y / 2,
            ),
            Position::Bot(pad) => Vec2D::new(frame.width / 2 - size.x / 2, pad),
            Position::Top(pad) => Vec2D::new(
                frame.width / 2 - size.x / 2,
                frame.height - size.y - pad,
            ),
        };

        Rect::new(pos, size)
    }
}

impl Default for Position {
    fn default() -> Self { Position::Center }
}
