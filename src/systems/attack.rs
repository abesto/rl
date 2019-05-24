use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;

use crate::components::*;
use crate::resources::messages::Messages;

pub struct AttackSystem;

#[derive(SystemData)]
pub struct AttackSystemData<'a> {
    living: WriteStorage<'a, Living>,
    power: ReadStorage<'a, Power>,
    name: ReadStorage<'a, Name>,
    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    messages: WriteExpect<'a, Messages>,
}

impl<'a> System<'a> for AttackSystem {
    type SystemData = AttackSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (attacker_pos, mut vel, attacker_name, attack_power) in
            (&data.position, &mut data.velocity, &data.name, &data.power)
                .join()
                .filter(|j| j.1.magnitude > 0)
        {
            let candidate = &*attacker_pos + &*vel;
            if let Some((_, target_living, target_name)) =
                (&data.position, &mut data.living, &data.name)
                    .join()
                    .find(|j| j.1.alive && j.0 == &candidate)
            {
                // a simple formula for attack damage
                let damage = attack_power.0 - target_living.defense;
                if damage > 0 {
                    // make the target take some damage
                    data.messages.push(
                        format!(
                            "{} attacks {} for {} hit points.",
                            attacker_name.0, target_name.0, damage
                        ),
                        colors::WHITE,
                    );
                    target_living.hp -= damage;
                } else {
                    data.messages.push(
                        format!(
                            "{} attacks {} but it has no effect!",
                            attacker_name.0, target_name.0
                        ),
                        colors::WHITE,
                    );
                }
                vel.magnitude = 0;
            }
        }
    }
}
