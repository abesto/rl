use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::{Position, Velocity};
use crate::map::Map;

pub struct MovementSystem;

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    position: WriteStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
    map: ReadExpect<'a, Map>,
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = MovementSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        for (pos, mut vel) in (&mut data.position, &mut data.velocity).join() {
            let orig_pos = pos.clone();
            while vel.magnitude != 0 {
                let candidate = &*pos + &*vel;
                if data.map[&candidate].blocked {
                    vel.magnitude = 0;
                    break;
                }
                *pos = candidate;
                vel.magnitude -= 1;
            }
        }
    }
}
