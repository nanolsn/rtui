use super::Vec2D;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T> {
    pub fn new<P, S>(pos: P, size: S) -> Self
        where
            P: Into<Vec2D<T>>,
            S: Into<Vec2D<T>>,
    {
        let pos = pos.into();
        let size = size.into();

        Rect {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    pub fn try_cast<U>(self) -> Option<Rect<U>>
        where
            T: num::NumCast,
            U: num::NumCast,
    {
        let x: Option<U> = num::cast(self.x);
        let y: Option<U> = num::cast(self.y);
        let width: Option<U> = num::cast(self.width);
        let height: Option<U> = num::cast(self.height);

        match (x, y, width, height) {
            (Some(x), Some(y), Some(w), Some(h)) => Some(Rect::new((x, y), (w, h))),
            _ => None,
        }
    }

    pub fn cast<U>(self) -> Rect<U>
        where
            T: num::NumCast,
            U: num::NumCast,
    { self.try_cast().expect("Some value can't be represented by the target type") }
}

#[allow(dead_code)]
impl<T> Rect<T>
    where
        T: Copy,
{
    pub fn pos(&self) -> Vec2D<T> { Vec2D::new(self.x, self.y) }

    pub fn size(&self) -> Vec2D<T> { Vec2D::new(self.width, self.height) }
}

impl<T> Rect<T>
    where
        T: Copy + num::Num,
{
    pub fn left(&self) -> T { self.x }

    pub fn right(&self) -> T { self.x + self.width }

    pub fn bot(&self) -> T { self.y }

    pub fn top(&self) -> T { self.y + self.height }

    pub fn center(&self) -> Vec2D<T> { self.pos() + self.size().half() }
}

impl<T> Rect<T>
    where
        T: Copy + num::Num + PartialOrd,
{
    pub fn intersects_point<P>(&self, point: P) -> bool
        where
            P: Into<Vec2D<T>>,
    {
        let point = point.into();

        self.left() <= point.x && point.x < self.right()
            && self.bot() <= point.y && point.y < self.top()
    }

    pub fn intersects_rect(&self, rhs: Rect<T>) -> bool {
        self.left() < rhs.right() && rhs.left() <= self.right()
            && self.bot() < rhs.top() && rhs.bot() <= self.top()
    }
}
