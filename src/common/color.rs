#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

#[allow(dead_code)]
impl Color {
    pub fn black() -> Self { Color(0.0, 0.0, 0.0, 1.0) }

    pub fn white() -> Self { Color(1.0, 1.0, 1.0, 1.0) }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self { Color(r, g, b, 1.0) }

    pub fn r(self) -> f32 { self.0 }
    pub fn g(self) -> f32 { self.1 }
    pub fn b(self) -> f32 { self.2 }
    pub fn a(self) -> f32 { self.3 }
}

impl Default for Color {
    fn default() -> Self { Color::black() }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self { Color::rgb(r, g, b) }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self { Color(r, g, b, a) }
}

impl From<glm::Vec3> for Color {
    fn from(v: glm::Vec3) -> Self { Color(v.x, v.y, v.z, 1.0) }
}

impl From<glm::Vec4> for Color {
    fn from(v: glm::Vec4) -> Self { Color(v.x, v.y, v.z, v.w) }
}
