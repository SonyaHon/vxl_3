use cgmath::prelude::*;
use specs::prelude::*;

pub struct Transform {}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Transform {}

impl Default for Transform {
    fn default() -> Self {
        Transform {}
    }
}
