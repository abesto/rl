use shred_derive::SystemData;
use specs::prelude::*;

use crate::{
    components::{Position, PreviousPosition},
    resources::state::State,
};

pub struct LocationHistorySystem;

#[derive(SystemData)]
pub struct LocationHistorySystemData<'a> {
    position: ReadStorage<'a, Position>,
    prev_position: WriteStorage<'a, PreviousPosition>,
    state: ReadExpect<'a, State>,
}

impl<'a> System<'a> for LocationHistorySystem {
    type SystemData = LocationHistorySystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.state != State::Game {
            return;
        }
        for (pos, prev_pos) in (&data.position, &mut data.prev_position).join() {
            *prev_pos = PreviousPosition { x: pos.x, y: pos.y };
        }
    }
}
