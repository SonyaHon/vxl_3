use specs::prelude::*;

pub struct Player;
impl Component for Player {
    type Storage = NullStorage<Self>;
}
impl Default for Player {
    fn default() -> Self {
        Player
    }
}
