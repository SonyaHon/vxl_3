use crate::vxl_gl::{gl, Gl};
use specs::prelude::*;

pub struct Mesh {
    vao_id: gl::types::GLuint,
    vbo_ids: Vec<gl::types::GLuint>,
}

impl Component for Mesh {
    type Storage = DenseVecStorage<Self>;
}

impl Mesh {
    pub fn from_data(gl: &Gl, vertices: Vec<cgmath::Vector3<f32>>, indices: Vec<i32>) -> Mesh {
        let vao_id: gl::types::GLuint = gl.create_vao();
        let vbo_ids = vec![
            gl.create_vertex_vbo(vao_id, vertices),
            gl.create_index_vbo(vao_id, indices),
        ];

        Mesh { vao_id, vbo_ids }
    }
}

impl Mesh {
    pub fn get_vao_id(&self) -> gl::types::GLuint {
        self.vao_id
    }

    pub fn get_vbo_ids(&self) -> &Vec<gl::types::GLuint> {
        &self.vbo_ids
    }
}
