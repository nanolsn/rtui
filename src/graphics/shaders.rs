use std::ffi::CStr;

use super::uniform::{
    Uniform,
    Accept,
    UniformError,
};

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

    pub fn check_compile_error(&self) -> Result<(), ShaderError> {
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

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn new(vs_code: &CStr, fs_code: &CStr) -> Result<ShaderProgram, ShaderError> {
        let vertex_shader = Shader::new_vertex_shader(vs_code)?;
        let fragment_shader = Shader::new_fragment_shader(fs_code)?;

        let shader_program_id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(shader_program_id, vertex_shader.id);
            gl::AttachShader(shader_program_id, fragment_shader.id);
            gl::LinkProgram(shader_program_id);
        }

        let shader_program = ShaderProgram { id: shader_program_id };

        shader_program.check_link_error()?;

        Ok(shader_program)
    }

    pub fn check_link_error(&self) -> Result<(), ShaderError> {
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

    pub fn use_program(&self) { unsafe { gl::UseProgram(self.id) } }

    pub fn get_uniform<T>(&self, name: T) -> i32
        where
            T: AsRef<CStr>,
    { unsafe { gl::GetUniformLocation(self.id, name.as_ref().as_ptr()) } }

    pub fn make_uniform<T, S>(&self, value: T, name: S) -> Result<Uniform<T>, UniformError>
        where
            T: Accept,
            S: AsRef<CStr>
    { Uniform::new(value, self.get_uniform(name)) }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::UseProgram(0);
            gl::DeleteProgram(self.id);
        }
    }
}
