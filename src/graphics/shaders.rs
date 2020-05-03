use std::ffi::CStr;

use super::{
    accept::Accept,
    shader_data::UsedShader,
    uniform::{
        Uniform,
        UniformError,
    },
};
use crate::graphics::uniform::SharedUniform;

const INFO_LOG_SIZE: usize = 512;

#[derive(Debug)]
pub enum ShaderError {
    IOError(std::io::Error),
    CompileError(std::ffi::CString),
    LinkError(std::ffi::CString),
}

impl From<std::io::Error> for ShaderError {
    fn from(e: std::io::Error) -> Self { ShaderError::IOError(e) }
}

#[derive(Debug)]
pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new_vertex_shader(code: &CStr) -> Result<Shader, ShaderError> {
        Shader::new(unsafe { gl::CreateShader(gl::VERTEX_SHADER) }, code)
    }

    pub fn new_fragment_shader(code: &CStr) -> Result<Shader, ShaderError> {
        Shader::new(unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) }, code)
    }

    fn new(shader_id: u32, code: &CStr) -> Result<Shader, ShaderError> {
        let shader = Shader { id: shader_id };

        unsafe {
            gl::ShaderSource(
                shader_id,
                1,
                &(code.as_ptr() as *const i8),
                std::ptr::null(),
            );

            gl::CompileShader(shader_id);
        }

        shader.check_compile_error()?;

        Ok(shader)
    }

    fn check_compile_error(&self) -> Result<(), ShaderError> {
        let mut success = gl::FALSE as i32;
        let mut info_log = Vec::with_capacity(INFO_LOG_SIZE);

        unsafe { gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success) }

        if success != gl::TRUE as i32 {
            unsafe {
                gl::GetShaderInfoLog(
                    self.id,
                    INFO_LOG_SIZE as i32,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut std::os::raw::c_char,
                );

                let err = CStr::from_ptr(info_log.as_ptr()).to_owned();
                Err(ShaderError::CompileError(err))
            }
        } else {
            Ok(())
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) { unsafe { gl::DeleteShader(self.id) } }
}

#[derive(Debug)]
pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    /// The `ShaderProgram` constructor.
    ///
    /// This function is unsafe, because a `ShaderProgram` needs to be properly deleted,
    /// but `ShaderProgram` doesn't implement `Drop`.
    unsafe fn new(vs_code: &CStr, fs_code: &CStr) -> Result<ShaderProgram, ShaderError> {
        let vertex_shader = Shader::new_vertex_shader(vs_code)?;
        let fragment_shader = Shader::new_fragment_shader(fs_code)?;

        let shader_program_id = gl::CreateProgram();

        gl::AttachShader(shader_program_id, vertex_shader.id);
        gl::AttachShader(shader_program_id, fragment_shader.id);
        gl::LinkProgram(shader_program_id);

        let shader_program = ShaderProgram { id: shader_program_id };
        shader_program.check_link_error()?;

        Ok(shader_program)
    }

    fn check_link_error(&self) -> Result<(), ShaderError> {
        let mut success = gl::FALSE as i32;
        let mut info_log = Vec::with_capacity(INFO_LOG_SIZE);

        unsafe { gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success) }

        if success != gl::TRUE as i32 {
            unsafe {
                gl::GetProgramInfoLog(
                    self.id,
                    INFO_LOG_SIZE as i32,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut std::os::raw::c_char,
                );

                let err = CStr::from_ptr(info_log.as_ptr()).to_owned();
                Err(ShaderError::LinkError(err))
            }
        } else {
            Ok(())
        }
    }

    pub fn id(&self) -> u32 { self.id }
}

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

    #[allow(dead_code)]
    pub fn len(&self) -> usize { self.shaders.len() }

    pub fn active(&self) -> Option<&ShaderProgram> {
        if let Some(idx) = self.used {
            Some(&self.shaders[idx])
        } else {
            None
        }
    }

    pub fn used(&self) -> Option<usize> { self.used }

    pub fn use_shader(&mut self, idx: usize) {
        match self.used {
            Some(i) if i == idx => return,
            _ => {
                unsafe { ShaderSet::use_shader_unsafe(self.shaders[idx].id) }
                self.used = Some(idx);
            }
        }
    }

    pub fn unuse_shader(&mut self) {
        if self.used.is_some() {
            unsafe { ShaderSet::use_shader_unsafe(0) }
            self.used = None;
        }
    }

    unsafe fn use_shader_unsafe(id: u32) { gl::UseProgram(id) }

    pub fn get_uniform<T>(&self, name: T) -> i32
        where
            T: AsRef<CStr>,
    {
        if let Some(shader) = self.active() {
            unsafe { ShaderSet::get_uniform_unsafe(shader.id, name.as_ref()) }
        } else {
            panic!("Shader is not used!")
        }
    }

    unsafe fn get_uniform_unsafe(shader_id: u32, name: &std::ffi::CStr) -> i32 {
        gl::GetUniformLocation(shader_id, name.as_ptr())
    }

    pub fn make_uniform<T, S>(&self, value: T, name: S) -> Result<Uniform<T>, UniformError>
        where
            T: Accept,
            S: AsRef<CStr>
    {
        if let Some(shader) = self.active() {
            Uniform::new(value, self.get_uniform(name), shader)
        } else {
            panic!("Shader is not used!")
        }
    }

    pub fn make_shared<T, S>(&mut self, value: T, name: S, used_shaders: &[UsedShader])
                             -> Result<SharedUniform<T>, UniformError>
        where
            T: Accept,
            S: AsRef<CStr>
    {
        let data = used_shaders
            .iter()
            .map(|&u| u as usize)
            .map(move |used|
                {
                    self.use_shader(used);
                    let shader_id = self.shaders[used].id;
                    let location = unsafe {
                        ShaderSet::get_uniform_unsafe(shader_id, name.as_ref())
                    };

                    (location, used)
                });

        Ok(SharedUniform::new(value, data)?)
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
