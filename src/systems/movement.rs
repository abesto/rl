use crate::components::{Position, Velocity};
use specs::{System, WriteStorage};

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (mut pos_storage, mut vel_storage): Self::SystemData) {
        use specs::Join;

        for (mut pos, mut vel) in (&mut pos_storage, &mut vel_storage).join() {
            pos.x += vel.x;
            pos.y += vel.y;
            vel.x = 0;
            vel.y = 0;
        }
    }
}
