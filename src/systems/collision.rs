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
            let mut cleared = 0;
            let mut blocked = false;
            while !blocked && cleared < vel.magnitude {
                let candidate = &*pos + &*vel;
                // First check for walls
                blocked = data.map[&candidate].blocked;
                // Check for objects blocking movement
                for (collider_pos, _) in (&data.collider_position, &data.collider).join() {
                    if collider_pos == &candidate {
                        blocked = true;
                    }
                }
                // If nothing blocked our path, we're cleared to move one tile further
                if !blocked {
                    cleared += 1;
                }
            }
            vel.magnitude = cleared;
        }
    }
}
