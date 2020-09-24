use cgmath::prelude::*;
use specs::prelude::*;

/// Component to track and change position of the Entity in the 3d world

#[allow(dead_code)]
pub struct Transform {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Vector3<f32>,
    scale: cgmath::Vector3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

#[allow(dead_code)]
impl Transform {
    pub fn from_data(
        position: cgmath::Vector3<f32>,
        rotation: cgmath::Vector3<f32>,
        scale: cgmath::Vector3<f32>,
    ) -> Transform {
        Transform {
            position,
            rotation,
            scale,
        }
    }

    pub fn from_position(position: cgmath::Vector3<f32>) -> Transform {
        Transform {
            position,
            rotation: cgmath::vec3(0.0, 0.0, 0.0),
            scale: cgmath::vec3(1.0, 1.0, 1.0),
        }
    }
}

#[allow(dead_code)]
impl Transform {
    pub fn set_position(&mut self, position: cgmath::Vector3<f32>) {
        self.position = position;
    }
    pub fn get_position(&self) -> cgmath::Vector3<f32> {
        self.position
    }

    pub fn set_rotation(&mut self, rotation: cgmath::Vector3<f32>) {
        self.rotation = rotation;
    }
    pub fn get_rotation(&self) -> cgmath::Vector3<f32> {
        self.rotation
    }

    pub fn set_scale(&mut self, scale: cgmath::Vector3<f32>) {
        self.scale = scale;
    }
    pub fn set_scale_uniform(&mut self, scale: f32) {
        self.scale = cgmath::vec3(scale, scale, scale);
    }
    pub fn get_scale(&self) -> cgmath::Vector3<f32> {
        self.scale
    }
}

#[allow(dead_code)]
impl Transform {
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position = self.position + cgmath::vec3(x, y, z);
    }

    pub fn translate_vector(&mut self, vector: cgmath::Vector3<f32>) {
        self.position = self.position + vector;
    }

    pub fn translate_x(&mut self, value: f32) {
        self.translate(value, 0.0, 0.0);
    }

    pub fn translate_y(&mut self, value: f32) {
        self.translate(0.0, value, 0.0);
    }

    pub fn translate_z(&mut self, value: f32) {
        self.translate(0.0, 0.0, value);
    }
}

#[allow(dead_code)]
impl Transform {
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = self.rotation + cgmath::vec3(x, y, z);
    }

    pub fn rotate_vector(&mut self, vector: cgmath::Vector3<f32>) {
        self.rotation = self.rotation + vector;
    }

    pub fn rotate_x(&mut self, value: f32) {
        self.rotate(value, 0.0, 0.0);
    }

    pub fn rotate_y(&mut self, value: f32) {
        self.rotate(0.0, value, 0.0);
    }

    pub fn rotate_z(&mut self, value: f32) {
        self.rotate(0.0, 0.0, value);
    }
}

#[allow(dead_code)]
impl Transform {
    pub fn get_transform_matrix(&self) -> cgmath::Matrix4<f32> {
        let mut transform_mat = cgmath::Matrix4::<f32>::identity();
        transform_mat = transform_mat * cgmath::Matrix4::<f32>::from_translation(self.position);
        transform_mat =
            transform_mat * cgmath::Matrix4::<f32>::from_angle_x(cgmath::Deg(self.rotation.x));
        transform_mat =
            transform_mat * cgmath::Matrix4::<f32>::from_angle_y(cgmath::Deg(self.rotation.y));
        transform_mat =
            transform_mat * cgmath::Matrix4::<f32>::from_angle_z(cgmath::Deg(self.rotation.z));
        transform_mat = transform_mat
            * cgmath::Matrix4::<f32>::from_nonuniform_scale(
                self.scale.x,
                self.scale.y,
                self.scale.z,
            );

        transform_mat
    }

    pub fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {
        let mut view_mat = cgmath::Matrix4::<f32>::identity();
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_angle_x(cgmath::Deg(self.rotation.x));
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_angle_y(cgmath::Deg(self.rotation.y));
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_angle_z(cgmath::Deg(self.rotation.z));
        view_mat = view_mat * cgmath::Matrix4::<f32>::from_translation(self.position * -1.0);

        view_mat
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: cgmath::vec3(0.0, 0.0, 0.0),
            rotation: cgmath::vec3(0.0, 0.0, 0.0),
            scale: cgmath::vec3(1.0, 1.0, 1.0),
        }
    }
}
