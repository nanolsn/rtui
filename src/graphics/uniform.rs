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
        if value == self.value {
            return;
        }

        self.value = value;
        self.accepted.set(false);
    }

    pub fn accept(&self, shader: &ShaderSet) {
        if self.accepted.get() {
            return;
        }

        match shader.active() {
            Some(shader) if shader.id() == self.shader => {
                self.value.accept(self.location);
                self.accepted.set(true);
            }
            _ => panic!("Shader {} is not used! (Used: {:?})", self.shader, shader.used()),
        }
    }
}

#[allow(dead_code)]
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
    pub fn new<D>(value: T, shader_data: D) -> Result<Self, UniformError>
        where
            D: IntoIterator<Item=(i32, usize)>,
    {
        let mut data = vec![];

        for (location, shader_idx) in shader_data.into_iter() {
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
        if value == self.value {
            return;
        }

        self.value = value;

        for data in &self.data {
            data.accepted.set(false);
        }
    }

    pub fn accept(&self, shader: &ShaderSet) {
        let used = shader.used().expect("Shader is not used!");

        for data in &self.data {
            if data.shader_idx == used {
                if !data.accepted.get() {
                    self.value.accept(data.location);
                    data.accepted.set(true);
                }

                break;
            }
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
