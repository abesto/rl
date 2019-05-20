use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::{BlocksMovement, Position, Velocity};
use crate::map::Map;

pub struct CollisionSystem;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    mover_position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    collider_position: ReadStorage<'a, Position>,
    collider: ReadStorage<'a, BlocksMovement>,

    map: ReadExpect<'a, Map>,
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        for (pos, mut vel) in (&data.mover_position, &mut data.velocity).join() {
            let candidate = &*pos + &*vel;
            let blocked = data
                .map
                .is_blocked(&candidate, (&data.collider_position, &data.collider).join());
            // If something blocks the movement, reject the whole thing
            if blocked {
                vel.magnitude = 0;
            }
        }
    }
}
