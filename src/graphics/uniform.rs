pub trait Accept {
    fn accept(&self, location: i32);
}

#[derive(Debug)]
pub enum UniformError {
    IncorrectLocation,
}

#[derive(Debug)]
pub struct Uniform<T>
    where
        T: Accept,
{
    value: T,
    location: i32,
    accepted: std::cell::Cell<bool>,
}

impl<T> Uniform<T>
    where
        T: Accept,
{
    pub fn new(value: T, location: i32) -> Result<Self, UniformError> {
        if location < 0 {
            return Err(UniformError::IncorrectLocation);
        }

        Ok(Uniform {
            value,
            location,
            accepted: std::cell::Cell::new(false),
        })
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.accepted.set(false);
    }

    #[allow(dead_code)]
    pub fn update_value<F>(&mut self, f: F)
        where
            F: FnOnce(&mut T),
    {
        f(&mut self.value);
        self.accepted.set(false);
    }

    pub fn accept(&self) {
        if !self.accepted.get() {
            self.value.accept(self.location);

            self.accepted.set(true);
        }
    }
}

impl<T> AsRef<T> for Uniform<T>
    where
        T: Accept,
{
    fn as_ref(&self) -> &T { &self.value }
}

impl<T> AsMut<T> for Uniform<T>
    where
        T: Accept,
{
    fn as_mut(&mut self) -> &mut T {
        self.accepted.set(false);
        &mut self.value
    }
}

impl Accept for glm::Vec1 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, self.x);
        }
    }
}

impl Accept for glm::Vec2 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform2f(location, self.x, self.y);
        }
    }
}

impl Accept for glm::Vec3 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.x, self.y, self.z);
        }
    }
}

impl Accept for glm::Vec4 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform4f(location, self.x, self.y, self.z, self.w);
        }
    }
}

impl Accept for glm::Mat2 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2fv(location, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl Accept for glm::Mat3 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl Accept for glm::Mat4 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl Accept for f32 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl Accept for i32 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl Accept for u32 {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform1ui(location, *self);
        }
    }
}

impl Accept for crate::common::color::Color {
    fn accept(&self, location: i32) {
        let [r, g, b, a] = self.into_f32();
        unsafe { gl::Uniform4f(location, r, g, b, a) }
    }
}
