use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::{Position, PreviousPosition};

pub struct MovementShadowSystem;

#[derive(SystemData)]
pub struct MovementShadowSystemData<'a> {
    position: ReadStorage<'a, Position>,
    prev_position: WriteStorage<'a, PreviousPosition>,
}

impl<'a> System<'a> for MovementShadowSystem {
    type SystemData = MovementShadowSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        for (pos, prev_pos) in (&data.position, &mut data.prev_position).join() {
            *prev_pos = PreviousPosition { x: pos.x, y: pos.y };
        }
    }
}
