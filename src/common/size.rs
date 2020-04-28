use super::Pos;

/// A size on the screen
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Size(pub u32, pub u32);

impl Size {
    pub fn as_pos(&self) -> Pos { Pos(self.0 as i32, self.1 as i32) }

    pub fn half(self) -> Size { Size(self.0 / 2, self.1 / 2) }
}

impl From<(i32, i32)> for Size {
    fn from((x, y): (i32, i32)) -> Self { Size(x as u32, y as u32) }
}

impl From<(u32, u32)> for Size {
    fn from((x, y): (u32, u32)) -> Self { Size(x, y) }
}

impl From<glm::Vec2> for Size {
    fn from(v: glm::Vec2) -> Self { Size(v.x as u32, v.y as u32) }
}

impl From<Pos> for Size {
    fn from(p: Pos) -> Self { p.as_size() }
}

impl std::ops::Add<Size> for Size {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output { Size(self.0 + rhs.0, self.1 + rhs.1) }
}

impl std::ops::Sub<Size> for Size {
    type Output = Size;

    fn sub(self, rhs: Size) -> Self::Output { Size(self.0 - rhs.0, self.1 - rhs.1) }
}

impl std::ops::Mul<u32> for Size {
    type Output = Size;

    fn mul(self, rhs: u32) -> Self::Output { Size(self.0 * rhs, self.1 * rhs) }
}

impl std::ops::Div<u32> for Size {
    type Output = Size;

    fn div(self, rhs: u32) -> Self::Output { Size(self.0 / rhs, self.1 / rhs) }
}

impl std::ops::Rem<u32> for Size {
    type Output = Size;

    fn rem(self, rhs: u32) -> Self::Output { Size(self.0 % rhs, self.1 % rhs) }
}

impl std::ops::MulAssign<u32> for Size {
    fn mul_assign(&mut self, rhs: u32) { *self = *self * rhs }
}

impl std::ops::DivAssign<u32> for Size {
    fn div_assign(&mut self, rhs: u32) { *self = *self / rhs }
}

impl std::ops::RemAssign<u32> for Size {
    fn rem_assign(&mut self, rhs: u32) { *self = *self % rhs }
}
