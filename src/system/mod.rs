use specs::prelude::*;

use crate::{component::transform::Transform, resource::DeltaTime};
pub struct TestSys;
impl<'a> System<'a> for TestSys {
    type SystemData = (ReadExpect<'a, DeltaTime>, WriteStorage<'a, Transform>);

    fn run(&mut self, (delta, mut transform): Self::SystemData) {
        for transform in (&mut transform).join() {
            transform.rotate_y(2.0 * delta.get_delta());
        }
    }
}
