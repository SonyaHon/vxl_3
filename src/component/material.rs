use specs::prelude::*;

pub struct Material {}
impl Component for Material {
    type Storage = DenseVecStorage<Self>;
}

impl Material {}

impl Default for Material {
    fn default() -> Self {
        Material {}
    }
}
