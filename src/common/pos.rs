/// A position on the screen
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos(pub i32, pub i32);

impl From<(i32, i32)> for Pos {
    fn from((x, y): (i32, i32)) -> Self { Pos(x, y) }
}

impl From<(u32, u32)> for Pos {
    fn from((x, y): (u32, u32)) -> Self { Pos(x as i32, y as i32) }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output { Pos(self.0 + rhs.0, self.1 + rhs.1) }
}

impl std::ops::Sub<Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output { Pos(self.0 - rhs.0, self.1 - rhs.1) }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output { Pos(self.0 * rhs, self.1 * rhs) }
}

impl std::ops::Div<i32> for Pos {
    type Output = Pos;

    fn div(self, rhs: i32) -> Self::Output { Pos(self.0 / rhs, self.1 / rhs) }
}

impl std::ops::Rem<i32> for Pos {
    type Output = Pos;

    fn rem(self, rhs: i32) -> Self::Output { Pos(self.0 % rhs, self.1 % rhs) }
}

impl std::ops::MulAssign<i32> for Pos {
    fn mul_assign(&mut self, rhs: i32) { *self = *self * rhs }
}

impl std::ops::DivAssign<i32> for Pos {
    fn div_assign(&mut self, rhs: i32) { *self = *self / rhs }
}

impl std::ops::RemAssign<i32> for Pos {
    fn rem_assign(&mut self, rhs: i32) { *self = *self % rhs }
}
