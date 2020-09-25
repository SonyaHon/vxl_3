use crate::vxl_gl::gl;

pub struct RenderTask {
    program_id: gl::types::GLuint,
    vao_id: gl::types::GLuint,
    vertex_count: i32,
    attrib_arrays: Vec<gl::types::GLuint>,
    mat4f_uniforms: Vec<(&'static str, cgmath::Matrix4<f32>)>,
    texture_id: Option<gl::types::GLuint>,
}
impl RenderTask {
    pub fn new(
        program_id: gl::types::GLuint,
        vao_id: gl::types::GLuint,
        vertex_count: i32,
        attrib_arrays: Vec<gl::types::GLuint>,
        mat4f_uniforms: Vec<(&'static str, cgmath::Matrix4<f32>)>,
        texture_id: Option<gl::types::GLuint>,
    ) -> Self {
        RenderTask {
            program_id,
            vao_id,
            vertex_count,
            attrib_arrays,
            mat4f_uniforms,
            texture_id,
        }
    }
    pub fn get_pid(&self) -> gl::types::GLuint {
        self.program_id
    }

    pub fn get_vao_id(&self) -> gl::types::GLuint {
        self.vao_id
    }

    pub fn get_vertex_count(&self) -> i32 {
        self.vertex_count
    }

    pub fn get_attri_arrays(&self) -> &Vec<gl::types::GLuint> {
        &self.attrib_arrays
    }

    pub fn get_mat4f_unifroms(&self) -> &Vec<(&str, cgmath::Matrix4<f32>)> {
        &self.mat4f_uniforms
    }

    pub fn get_texture_id(&self) -> Option<gl::types::GLuint> {
        self.texture_id
    }
}

pub struct MainCameraTask {
    projection_mat: cgmath::Matrix4<f32>,
    view_mat: cgmath::Matrix4<f32>,
}

impl MainCameraTask {
    pub fn new(projection_mat: cgmath::Matrix4<f32>, view_mat: cgmath::Matrix4<f32>) -> Self {
        MainCameraTask {
            projection_mat,
            view_mat,
        }
    }

    pub fn get_projection_mat(&self) -> cgmath::Matrix4<f32> {
        self.projection_mat
    }

    pub fn get_view_mat(&self) -> cgmath::Matrix4<f32> {
        self.view_mat
    }
}
