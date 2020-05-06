use super::Rect;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self { Vec2D { x, y } }

    pub fn try_cast<U>(self) -> Option<Vec2D<U>>
        where
            T: num::NumCast,
            U: num::NumCast,
    {
        let x: Option<U> = num::cast(self.x);
        let y: Option<U> = num::cast(self.y);

        match (x, y) {
            (Some(x), Some(y)) => Some(Vec2D::new(x, y)),
            _ => None,
        }
    }

    pub fn cast<U>(self) -> Vec2D<U>
        where
            T: num::NumCast,
            U: num::NumCast,
    { self.try_cast().expect("Some value can't be represented by the target type") }

    pub fn into_inner(self) -> (T, T) { (self.x, self.y) }

    pub fn into_rect<U>(self) -> Rect<U>
        where
            T: num::NumCast,
            U: num::NumCast + num::Zero,
    { Rect::new((U::zero(), U::zero()), self.cast()) }
}

impl<T> Vec2D<T>
    where
        T: Copy,
{
    pub fn width(&self) -> T { self.x }
    pub fn height(&self) -> T { self.y }
}

impl<T> Vec2D<T>
    where
        T: Copy + num::Num,
{
    pub fn half(self) -> Vec2D<T> { self / (T::one() + T::one()) }
}

impl<T> From<[T; 2]> for Vec2D<T> {
    fn from([x, y]: [T; 2]) -> Self { Vec2D::new(x, y) }
}

impl<T> From<(T, T)> for Vec2D<T> {
    fn from((x, y): (T, T)) -> Self { Vec2D::new(x, y) }
}

impl<T> From<glm::TVec2<T>> for Vec2D<T>
    where
        T: PartialEq + Copy + std::fmt::Debug + 'static,
{
    fn from(v: glm::TVec2<T>) -> Self { Vec2D::new(v.x, v.y) }
}

impl<T, R> std::ops::Add<R> for Vec2D<T>
    where
        R: Into<Vec2D<T>>,
        T: std::ops::Add<T>,
{
    type Output = Vec2D<T::Output>;

    fn add(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Vec2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T, R> std::ops::Sub<R> for Vec2D<T>
    where
        R: Into<Vec2D<T>>,
        T: std::ops::Sub<T>,
{
    type Output = Vec2D<T::Output>;

    fn sub(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Vec2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T, R> std::ops::AddAssign<R> for Vec2D<T>
    where
        R: Into<Vec2D<T>>,
        T: std::ops::Add<T, Output=T> + Copy,
{
    fn add_assign(&mut self, rhs: R) { *self = *self + rhs }
}

impl<T, R> std::ops::SubAssign<R> for Vec2D<T>
    where
        R: Into<Vec2D<T>>,
        T: std::ops::Sub<T, Output=T> + Copy,
{
    fn sub_assign(&mut self, rhs: R) { *self = *self - rhs }
}

impl<T> std::ops::Mul<T> for Vec2D<T>
    where
        T: std::ops::Mul<T> + Copy,
{
    type Output = Vec2D<T::Output>;

    fn mul(self, rhs: T) -> Self::Output { Vec2D::new(self.x * rhs, self.y * rhs) }
}

impl<T> std::ops::Div<T> for Vec2D<T>
    where
        T: std::ops::Div<T> + Copy,
{
    type Output = Vec2D<T::Output>;

    fn div(self, rhs: T) -> Self::Output { Vec2D::new(self.x / rhs, self.y / rhs) }
}

impl<T> std::ops::Rem<T> for Vec2D<T>
    where
        T: std::ops::Rem<T> + Copy,
{
    type Output = Vec2D<T::Output>;

    fn rem(self, rhs: T) -> Self::Output { Vec2D::new(self.x % rhs, self.y % rhs) }
}

impl<T> std::ops::MulAssign<T> for Vec2D<T>
    where
        T: std::ops::Mul<T, Output=T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) { *self = *self * rhs }
}

impl<T> std::ops::DivAssign<T> for Vec2D<T>
    where
        T: std::ops::Div<T, Output=T> + Copy,
{
    fn div_assign(&mut self, rhs: T) { *self = *self / rhs }
}

impl<T> std::ops::RemAssign<T> for Vec2D<T>
    where
        T: std::ops::Rem<T, Output=T> + Copy,
{
    fn rem_assign(&mut self, rhs: T) { *self = *self % rhs }
}
