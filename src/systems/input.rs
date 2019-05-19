use specs::{ReadExpect, System, WriteStorage};
use tcod::input::Key;

use crate::components::Velocity;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadExpect<'a, Option<Key>>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (key, mut vel_storage): Self::SystemData) {
        use specs::Join;
        use tcod::input::KeyCode::*;

        for vel in (&mut vel_storage).join() {
            key.map(|k| match k {
                Key { code: Up, .. } => vel.y -= 1,
                Key { code: Down, .. } => vel.y += 1,
                Key { code: Left, .. } => vel.x -= 1,
                Key { code: Right, .. } => vel.x += 1,
                _ => (),
            });
        }
    }
}
