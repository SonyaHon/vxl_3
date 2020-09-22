use cgmath::prelude::*;
use specs::prelude::*;

/// Component to track and change position of the Entity in the 3d world
pub struct Transform {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Vector3<f32>,
    scale: cgmath::Vector3<f32>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

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

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: cgmath::vec3(0.0, 0.0, 0.0),
            rotation: cgmath::vec3(0.0, 0.0, 0.0),
            scale: cgmath::vec3(1.0, 1.0, 1.0),
        }
    }
}
