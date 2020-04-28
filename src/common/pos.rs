use super::Size;

/// A position on the screen
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn as_size(&self) -> Size { Size(self.0 as u32, self.1 as u32) }

    pub fn x(&self) -> i32 { self.0 }
    pub fn y(&self) -> i32 { self.1 }
}

impl From<(i32, i32)> for Pos {
    fn from((x, y): (i32, i32)) -> Self { Pos(x, y) }
}

impl From<(u32, u32)> for Pos {
    fn from((x, y): (u32, u32)) -> Self { Pos(x as i32, y as i32) }
}

impl From<(f32, f32)> for Pos {
    fn from((x, y): (f32, f32)) -> Self { Pos(x as i32, y as i32) }
}

impl From<glm::Vec2> for Pos {
    fn from(v: glm::Vec2) -> Self { Pos(v.x as i32, v.y as i32) }
}

impl From<Size> for Pos {
    fn from(s: Size) -> Self { s.as_pos() }
}

impl<P> std::ops::Add<P> for Pos
    where
        P: Into<Pos>,
{
    type Output = Pos;

    fn add(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<P> std::ops::Sub<P> for Pos
    where
        P: Into<Pos>,
{
    type Output = Pos;

    fn sub(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<P> std::ops::AddAssign<P> for Pos
    where
        P: Into<Pos>,
{
    fn add_assign(&mut self, rhs: P) {
        let rhs = rhs.into();
        *self = *self + rhs
    }
}

impl<P> std::ops::SubAssign<P> for Pos
    where
        P: Into<Pos>,
{
    fn sub_assign(&mut self, rhs: P) {
        let rhs = rhs.into();
        *self = *self - rhs
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output { Pos(self.0 * rhs, self.1 * rhs) }
}

impl std::ops::Div<i32> for Pos {
    type Output = Pos;

    fn div(self, rhs: i32) -> Self::Output { Pos(self.0 / rhs, self.1 / rhs) }
}