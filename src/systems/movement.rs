use shred_derive::SystemData;
use specs::prelude::*;

use crate::{
    components::{Position, Velocity},
    resources::state::State,
};

pub struct MovementSystem;

#[derive(SystemData)]
pub struct MovementSystemData<'a> {
    position: WriteStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
    state: ReadExpect<'a, State>,
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = MovementSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.state != State::Game {
            return;
        }
        for (pos, mut vel) in (&mut data.position, &mut data.velocity).join() {
            *pos = &*pos + &*vel;
            vel.magnitude = 0;
        }
    }
}
