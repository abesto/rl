use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::*;

pub struct AttackSystem;

#[derive(SystemData)]
pub struct AttackSystemData<'a> {
    alive: ReadStorage<'a, Alive>,
    fighter: ReadStorage<'a, Fighter>,
    name: ReadStorage<'a, Name>,
    player: ReadStorage<'a, Player>,
    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    entity: Entities<'a>,
}

impl<'a> System<'a> for AttackSystem {
    type SystemData = AttackSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        for (attacker_pos, mut vel, attacker_name, attacker, _) in (
            &data.position,
            &mut data.velocity,
            &data.name,
            &data.entity,
            &data.fighter,
        )
            .join()
        {
            if vel.magnitude == 0 {
                continue;
            }
            let candidate = &*attacker_pos + &*vel;
            let is_attacker_player = data.player.get(attacker).is_some();
            for (target_pos, alive, target_name, target, _) in (
                &data.position,
                &data.alive,
                &data.name,
                &data.entity,
                &data.fighter,
            )
                .join()
            {
                if alive.0 && target_pos == &candidate {
                    let is_target_player = data.player.get(target).is_some();
                    if is_attacker_player {
                        println!(
                            "The {} laughs at your puny efforts to attack him!",
                            target_name.0
                        );
                    } else if is_target_player {
                        println!(
                            "The attack of the {} bounces off your shiny metal armor!",
                            attacker_name.0
                        );
                    } else {
                        println!(
                            "The confused {} swings at the {}!",
                            attacker_name.0, target_name.0
                        );
                    }
                    vel.magnitude = 0;
                    break;
                }
            }
        }
    }
}
