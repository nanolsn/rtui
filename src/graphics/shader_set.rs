use std::ffi::CStr;

use super::shaders::{
    ShaderProgram,
    ShaderError,
};
use std::ops::Index;

#[derive(Debug)]
pub struct ShaderSet {
    shaders: Vec<ShaderProgram>,
    used: Option<usize>,
}

impl ShaderSet {
    pub fn new() -> Self {
        ShaderSet {
            shaders: vec![],
            used: None,
        }
    }

    pub fn add(&mut self, vs_code: &CStr, fs_code: &CStr) -> Result<(), ShaderError> {
        let shader = unsafe { ShaderProgram::new(vs_code, fs_code)? };
        self.shaders.push(shader);

        Ok(())
    }

    pub fn use_shader(&mut self, idx: usize) {
        match self.used {
            Some(i) if i == idx => return,
            _ => {
                unsafe { gl::UseProgram(self.shaders[idx].id) }
                self.used = Some(idx);
            }
        }
    }

    pub fn unuse_shader(&mut self) {
        if self.used.is_some() {
            unsafe { gl::UseProgram(0) };
            self.used = None;
        }
    }
}

impl std::ops::Index<usize> for ShaderSet {
    type Output = ShaderProgram;

    fn index(&self, index: usize) -> &Self::Output { &self.shaders[index] }
}

impl Drop for ShaderSet {
    fn drop(&mut self) {
        self.unuse_shader();

        for shader in self.shaders.iter() {
            unsafe { gl::DeleteProgram(shader.id) }
        }
    }
}
