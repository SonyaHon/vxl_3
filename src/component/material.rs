use specs::prelude::*;

use crate::{loader::shaders::ShaderManager, loader::textures::texture::Texture, vxl_gl::gl};

pub struct Material {
    shader_program_id: gl::types::GLuint,
    texture_id: Option<gl::types::GLuint>,
}
impl Component for Material {
    type Storage = DenseVecStorage<Self>;
}

impl Material {
    pub fn default(shader_loader: &ShaderManager) -> Material {
        let program = shader_loader.get_shader_program("default");

        Material {
            shader_program_id: program.get_id(),
            texture_id: None,
        }
    }

    pub fn add_texture(&mut self, texture: &Texture) {
        self.texture_id = Some(texture.get_id());
    }
}

impl Material {
    pub fn get_program_id(&self) -> gl::types::GLuint {
        self.shader_program_id
    }
}
