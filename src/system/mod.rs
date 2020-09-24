use specs::prelude::*;

use crate::component::transform::Transform;
pub struct TestSys;
impl<'a> System<'a> for TestSys {
    type SystemData = WriteStorage<'a, Transform>;

    fn run(&mut self, mut transform: Self::SystemData) {
        for transform in (&mut transform).join() {
            transform.rotate_y(0.02);
        }
    }
}
