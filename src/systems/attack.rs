use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::*;

pub struct AttackSystem;

#[derive(SystemData)]
pub struct AttackSystemData<'a> {
    player: ReadStorage<'a, Player>,
    player_position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    target_position: ReadStorage<'a, Position>,
    alive: ReadStorage<'a, Alive>,
    name: ReadStorage<'a, Name>,
}

impl<'a> System<'a> for AttackSystem {
    type SystemData = AttackSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        for (pos, mut vel, _) in (&data.player_position, &mut data.velocity, &data.player).join() {
            let candidate = &*pos + &*vel;
            for (target_pos, alive, name, _) in (
                &data.target_position,
                &data.alive,
                &data.name,
                !&data.player,
            )
                .join()
            {
                if alive.0 && target_pos == &candidate {
                    println!("The {} laughs at your puny efforts to attack him!", name.0);
                    vel.magnitude = 0;
                    break;
                }
            }
        }
    }
}
