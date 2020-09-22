use specs::prelude::*;

pub struct TestSystem;
impl<'a> System<'a> for TestSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        println!("Hello SPECS");
    }
}
