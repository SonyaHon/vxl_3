use specs::prelude::*;

use crate::vxl_gl::gl;

pub struct Material {
    shader_program_id: gl::types::GLuint,
    shaders_ids: gl::types::GLuint,
}
impl Component for Material {
    type Storage = DenseVecStorage<Self>;
}

impl Material {}

impl Default for Material {
    fn default() -> Self {
        Material {}
    }
}
