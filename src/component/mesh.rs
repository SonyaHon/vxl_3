use crate::vxl_gl::{gl, Gl};
use specs::prelude::*;

/// Component that creates and holds vao and vbos of the mesh
pub struct Mesh {
    vao_id: gl::types::GLuint,
    vertex_count: i32,
    attrib_arays: Vec<gl::types::GLuint>,
    has_uvs: bool,
}

impl Component for Mesh {
    type Storage = DenseVecStorage<Self>;
}

impl Mesh {
    pub fn from_data(gl: &Gl, vertices: Vec<cgmath::Vector3<f32>>, indices: Vec<u32>) -> Mesh {
        let vertex_count = indices.len() as i32;
        let vao_id: gl::types::GLuint = gl.create_vao();
        gl.bind_vao(vao_id);
        gl.create_index_vbo(indices);
        gl.create_vertex_vbo(vertices);
        gl.unbind_vao();

        let attrib_arays: Vec<gl::types::GLuint> = vec![0];

        Mesh {
            vao_id,
            vertex_count,
            attrib_arays,
            has_uvs: false,
        }
    }

    pub fn add_uvs(&mut self, gl: &Gl, uvs: Vec<cgmath::Vector2<f32>>) {
        self.has_uvs = true;
        self.attrib_arays.push(1);
        gl.bind_vao(self.vao_id);
        gl.create_uvs_vbo(uvs);
        gl.unbind_vao();
    }
}

impl Mesh {
    pub fn get_vao_id(&self) -> gl::types::GLuint {
        self.vao_id
    }

    pub fn get_vertex_count(&self) -> i32 {
        self.vertex_count
    }

    pub fn get_attrib_arrays(&self) -> &Vec<gl::types::GLuint> {
        &self.attrib_arays
    }
}
