use specs::prelude::*;

pub struct Camera {
    fovy: f32,
    aspect: f32,
    near_plane: f32,
    far_plane: f32,
}
impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}

impl Camera {
    pub fn new(fovy: f32, aspect: f32, near_plane: f32, far_plane: f32) -> Camera {
        Camera {
            fovy,
            aspect,
            near_plane,
            far_plane,
        }
    }

    pub fn get_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let mat = cgmath::perspective(
            cgmath::Deg(self.fovy),
            self.aspect,
            self.near_plane,
            self.far_plane,
        );

        mat
    }
}

pub struct MainCamera;
impl Component for MainCamera {
    type Storage = NullStorage<Self>;
}
impl Default for MainCamera {
    fn default() -> Self {
        MainCamera
    }
}
