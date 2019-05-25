use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::{Collider, Position, Velocity};
use crate::resources::map::{CalculateBlockedMapExt, Map};

pub struct CollisionSystem;

#[derive(SystemData)]
pub struct CollisionSystemData<'a> {
    collider: ReadStorage<'a, Collider>,
    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    map: Option<ReadExpect<'a, Map>>,
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = CollisionSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.map.is_none() {
            return;
        }
        for (pos, mut vel) in (&data.position, &mut data.velocity).join() {
            if vel.magnitude == 0 {
                continue;
            }
            let candidate = &*pos + &*vel;
            let blocked = data
                .map
                .as_ref()
                .unwrap()
                .is_blocked(&candidate, (&data.position, &data.collider).join());
            // If something blocks the movement, reject the whole thing
            if blocked {
                vel.magnitude = 0;
            }
        }
    }
}
