use specs::prelude::*;

use crate::component::{player::Player, transform::Transform};
use crate::resource::DeltaTime;

pub struct DemoPlayerRotationSys;
impl<'a> System<'a> for DemoPlayerRotationSys {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, DeltaTime>,
    );

    fn run(&mut self, (player, mut transfrom, delta): Self::SystemData) {
        let (_, transform) = (&player, &mut transfrom).join().next().unwrap();

        const ROTATION_SPEED: f32 = 0.ear;

        transform.rotate_y(ROTATION_SPEED * delta.get_delta());
    }
}
