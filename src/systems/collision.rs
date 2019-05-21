use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::{Collider, Position, Velocity};
use crate::map::Map;

pub struct CollisionSystem;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
    collider: ReadStorage<'a, Collider>,

    map: ReadExpect<'a, Map>,
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        for (pos, mut vel) in (&data.position, &mut data.velocity).join() {
            if vel.magnitude == 0 {
                continue;
            }
            let candidate = &*pos + &*vel;
            let blocked = data
                .map
                .is_blocked(&candidate, (&data.position, &data.collider).join());
            // If something blocks the movement, reject the whole thing
            if blocked {
                vel.magnitude = 0;
            }
        }
    }
}
