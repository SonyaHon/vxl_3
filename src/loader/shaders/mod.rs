use shader::Shader;

use crate::vxl_gl::{gl, Gl};

use super::Loader;

pub mod shader;
pub mod shader_program;

pub struct ShaderLoader<'a> {
    gl: &'a Gl,
    programs: std::collections::HashMap<&'static str, shader_program::ShaderProgram>,
    loader: Loader,
}

impl<'a> ShaderLoader<'a> {
    pub fn new(gl: &'a Gl) -> ShaderLoader {
        let loader = Loader::new("shaders/");

        ShaderLoader {
            gl,
            programs: std::collections::HashMap::new(),
            loader,
        }
    }

    pub fn add_shader_program(
        &mut self,
        program_name: &'static str,
        shaders: Vec<(&'static str, gl::types::GLenum)>,
    ) -> &mut Self {
        let mut built_shaders: Vec<Shader> = Vec::with_capacity(shaders.len());

        for (asset_path, shader_type) in shaders {
            let shader_source = self.loader.load_as_cstring(asset_path);
            let shader = Shader::from_source(self.gl, &shader_source, shader_type);

            built_shaders.push(shader);
        }

        let program = shader_program::ShaderProgram::from_shaders(self.gl, &built_shaders);
        self.programs.entry(program_name).or_insert(program);

        println!("Shader \"{}\" - Loaded", program_name);

        self
    }

    pub fn finish(&mut self) -> ShaderManager {
        let mut hash_copy: std::collections::HashMap<&'static str, shader_program::ShaderProgram> =
            std::collections::HashMap::new();

        self.programs.iter().for_each(|(key, value)| {
            hash_copy
                .entry(key)
                .or_insert(shader_program::ShaderProgram::from(value));
        });
        ShaderManager::new(hash_copy)
    }
}

pub struct ShaderManager {
    programs: std::collections::HashMap<&'static str, shader_program::ShaderProgram>,
}

impl ShaderManager {
    pub fn new(
        programs: std::collections::HashMap<&'static str, shader_program::ShaderProgram>,
    ) -> ShaderManager {
        ShaderManager { programs }
    }

    pub fn get_shader_program(&self, program_name: &'static str) -> &shader_program::ShaderProgram {
        &self.programs.get(program_name).unwrap()
    }
}
