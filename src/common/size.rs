/// A size on the screen
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Size(pub u32, pub u32);

impl From<(i32, i32)> for Size {
    fn from((x, y): (i32, i32)) -> Self { Size(x as u32, y as u32) }
}

impl From<(u32, u32)> for Size {
    fn from((x, y): (u32, u32)) -> Self { Size(x, y) }
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
