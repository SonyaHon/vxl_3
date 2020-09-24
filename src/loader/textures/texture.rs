use crate::vxl_gl::gl;

pub struct Texture {
    dimensions: cgmath::Vector2<u32>,
    id: gl::types::GLuint,
}

impl Texture {
    pub fn new(dimensions: cgmath::Vector2<u32>, id: gl::types::GLuint) -> Self {
        Texture { id, dimensions }
    }

    pub fn get_dimensions(&self) -> cgmath::Vector2<u32> {
        self.dimensions
    }

    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl From<&Texture> for Texture {
    fn from(value: &Texture) -> Self {
        Texture {
            id: value.get_id(),
            dimensions: value.get_dimensions(),
        }
    }
}
