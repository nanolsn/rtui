#[derive(Debug)]
pub enum GLError {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    InvalidFramebufferOperation,
    ContextLost,
}

impl GLError {
    fn from_gl_error(code: u32) -> GLError {
        match code {
            gl::INVALID_ENUM => GLError::InvalidEnum,
            gl::INVALID_VALUE => GLError::InvalidValue,
            gl::INVALID_OPERATION => GLError::InvalidOperation,
            gl::STACK_OVERFLOW => GLError::StackOverflow,
            gl::STACK_UNDERFLOW => GLError::StackUnderflow,
            gl::OUT_OF_MEMORY => GLError::OutOfMemory,
            gl::INVALID_FRAMEBUFFER_OPERATION => GLError::InvalidFramebufferOperation,
            gl::CONTEXT_LOST => GLError::ContextLost,
            _ => panic!("Undefined error code!"),
        }
    }
}

pub fn check_error() -> Result<(), GLError> {
    unsafe {
        let err = gl::GetError();

        if err == gl::NO_ERROR {
            Ok(())
        } else {
            Err(GLError::from_gl_error(err))
        }
    }
}

pub fn unwrap_error() { check_error().unwrap() }
