pub trait Accept: PartialEq {
    fn accept(&self, location: i32);
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

impl Accept for bool {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, (*self).into());
        }
    }
}

impl Accept for crate::common::Color {
    fn accept(&self, location: i32) {
        unsafe {
            gl::Uniform4f(location, self.0, self.1, self.2, self.3);
        }
    }
}
