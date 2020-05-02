use super::{
    accept::Accept,
    shaders::{
        ShaderProgram,
        ShaderSet,
    },
};

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
    shader: u32,
    accepted: std::cell::Cell<bool>,
}

impl<T> Uniform<T>
    where
        T: Accept,
{
    pub fn new(value: T, location: i32, shader: &ShaderProgram) -> Result<Self, UniformError> {
        if location < 0 {
            return Err(UniformError::IncorrectLocation);
        }

        Ok(Uniform {
            value,
            location,
            shader: shader.id(),
            accepted: std::cell::Cell::new(false),
        })
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.accepted.set(false);
    }

    pub fn accept(&self, shader: &ShaderSet) {
        if !self.accepted.get() {
            self.direct_accept(shader);
        }
    }

    pub fn set(&mut self, value: T, shader: &ShaderSet) {
        self.value = value;
        self.direct_accept(shader);
    }

    fn direct_accept(&self, shader: &ShaderSet) {
        match shader.active() {
            Some(shader) if shader.id() == self.shader => {
                self.value.accept(self.location);
                self.accepted.set(true);
            }
            _ => panic!("Shader {} is not used! (Used: {:?})", self.shader, shader.used()),
        }
    }
}

impl<T> Uniform<T>
    where
        T: Copy + Accept,
{
    pub fn get(&self) -> T { self.value }
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

#[derive(Debug)]
pub struct UniformData {
    location: i32,
    shader_idx: usize,
}

#[derive(Debug)]
pub struct SharedUniform<T>
    where
        T: Accept,
{
    value: T,
    data: Vec<UniformData>,
    accepted: std::cell::Cell<bool>,
}

impl<T> SharedUniform<T>
    where
        T: Accept,
{
    pub fn new(value: T, shader_data: Vec<(i32, usize)>) -> Result<Self, UniformError> {
        let mut data = vec![];

        for (location, shader_idx) in shader_data {
            if location < 0 {
                return Err(UniformError::IncorrectLocation);
            }

            data.push(UniformData {
                location,
                shader_idx,
            });
        }

        Ok(SharedUniform {
            value,
            data,
            accepted: std::cell::Cell::new(false),
        })
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.accepted.set(false);
    }

    pub fn accept(&self, shader: &mut ShaderSet) {
        if !self.accepted.get() {
            self.direct_accept(shader);
        }
    }

    pub fn set(&mut self, value: T, shader: &mut ShaderSet) {
        self.value = value;
        self.direct_accept(shader);
    }

    fn direct_accept(&self, shader: &mut ShaderSet) {
        for data in self.data.iter() {
            shader.use_shader(data.shader_idx);
            self.value.accept(data.location);
        }

        self.accepted.set(true);
    }
}

impl<T> SharedUniform<T>
    where
        T: Copy + Accept,
{
    pub fn get(&self) -> T { self.value }
}

impl<T> AsRef<T> for SharedUniform<T>
    where
        T: Accept,
{
    fn as_ref(&self) -> &T { &self.value }
}

impl<T> AsMut<T> for SharedUniform<T>
    where
        T: Accept,
{
    fn as_mut(&mut self) -> &mut T {
        self.accepted.set(false);
        &mut self.value
    }
}
