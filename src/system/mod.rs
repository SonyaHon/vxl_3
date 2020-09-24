use specs::prelude::*;

use crate::{component::camera::MainCamera, component::transform::Transform, resource::DeltaTime};
pub struct TestSys;
impl<'a> System<'a> for TestSys {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        ReadStorage<'a, MainCamera>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (delta, main_camera, mut transform): Self::SystemData) {
        for (_, transform) in (&main_camera, &mut transform).join() {
            transform.translate_z(0.02 * delta.get_delta());
        }
    }
}
