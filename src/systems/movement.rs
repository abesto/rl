use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::{Position, Velocity};

pub struct MovementSystem;

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    player_position: WriteStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = MovementSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        for (pos, mut vel) in (&mut data.player_position, &mut data.velocity).join() {
            *pos = &*pos + &*vel;
            vel.magnitude = 0;
        }
    }
}
