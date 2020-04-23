#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Color {
    col: [u8; 4],
}

impl Color {
    pub fn black() -> Self { Color { col: [0, 0, 0, !0] } }

    pub fn white() -> Self { Color { col: [!0, !0, !0, !0] } }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self { Color { col: [r, g, b, !0] } }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { Color { col: [r, g, b, a] } }

    pub fn into_f32(self) -> [f32; 4] { [self.r(), self.g(), self.b(), self.a()] }

    pub fn r(self) -> f32 { self.col[0] as f32 / std::u8::MAX as f32 }
    pub fn g(self) -> f32 { self.col[1] as f32 / std::u8::MAX as f32 }
    pub fn b(self) -> f32 { self.col[2] as f32 / std::u8::MAX as f32 }
    pub fn a(self) -> f32 { self.col[3] as f32 / std::u8::MAX as f32 }
}

impl Default for Color {
    fn default() -> Self { Color::black() }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self { Color::rgb(r, g, b) }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self { Color::rgba(r, g, b, a) }
}
