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
    accepted: std::cell::Cell<bool>,
}

#[derive(Debug)]
pub struct SharedUniform<T>
    where
        T: Accept,
{
    value: T,
    data: Vec<UniformData>,
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
                accepted: std::cell::Cell::new(false),
            });
        }

        Ok(SharedUniform { value, data })
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;

        for data in &self.data {
            data.accepted.set(false);
        }
    }

    pub fn accept(&self, shader: &ShaderSet) {
        match shader.used() {
            Some(used) => {
                if !self.data[used].accepted.get() {
                    self.direct_accept(shader);
                }
            },
            _ => panic!("Shader is not used!"),
        }
    }

    pub fn set(&mut self, value: T, shader: &ShaderSet) {
        self.value = value;
        self.direct_accept(shader);
    }

    fn direct_accept(&self, shader: &ShaderSet) {
        match shader.used() {
            Some(used) => {
                self.value.accept(self.data[used].location);
                self.data[used].accepted.set(true);
            }
            _ => panic!("Shader not used!"),
        }
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
        for data in &self.data {
            data.accepted.set(false);
        }

        &mut self.value
    }
}
