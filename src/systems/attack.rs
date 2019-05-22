use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::*;

pub struct AttackSystem;

#[derive(SystemData)]
pub struct AttackSystemData<'a> {
    living: WriteStorage<'a, Living>,
    power: ReadStorage<'a, Power>,
    name: ReadStorage<'a, Name>,
    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
}

impl<'a> System<'a> for AttackSystem {
    type SystemData = AttackSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

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
                    println!(
                        "{} attacks {} for {} hit points.",
                        attacker_name.0, target_name.0, damage
                    );
                    target_living.take_damage(damage);
                } else {
                    println!(
                        "{} attacks {} but it has no effect!",
                        attacker_name.0, target_name.0
                    );
                }
                vel.magnitude = 0;
            }
        }
    }
}
