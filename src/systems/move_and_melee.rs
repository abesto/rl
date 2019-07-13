use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;

use crate::{
    components::*,
    resources::{messages::Messages, state::State},
};

pub struct MoveAndMeleeSystem;

#[derive(SystemData)]
pub struct MoveAndMeleeSystemData<'a> {
    living: WriteStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    power: ReadStorage<'a, Power>,
    name: ReadStorage<'a, Name>,
    position: ReadStorage<'a, Position>,
    action: WriteStorage<'a, Action>,
    energy: WriteStorage<'a, Energy>,
    velocity: WriteStorage<'a, Velocity>,

    entity: Entities<'a>,
    state: WriteExpect<'a, State>,
    messages: Write<'a, Messages>,
}

impl<'a> System<'a> for MoveAndMeleeSystem {
    type SystemData = MoveAndMeleeSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.state != State::Game {
            return;
        }

        for (attacker_pos, attacker_velocity, action, energy, attacker_name, attack_power) in (
            &data.position,
            &mut data.velocity,
            &mut data.action,
            &mut data.energy,
            &data.name,
            &data.power,
        )
            .join()
        {
            let energy_cost = action.energy_cost();
            match action {
                Action::MoveOrMelee {
                    velocity,
                    attack_monsters,
                    attack_player,
                } => {
                    let candidate = &*attacker_pos + &velocity;
                    if let Some((_, target_living, target_name, target_entity)) =
                        (&data.position, &mut data.living, &data.name, &data.entity)
                            .join()
                            .find(|j| j.1.alive && j.0 == &candidate)
                    {
                        if energy.consume(energy_cost) {
                            // Check if the attacker wants to attack the target
                            let is_target_player = data.player.get(target_entity).is_some();
                            if (is_target_player && !*attack_player)
                                || (!is_target_player && !*attack_monsters)
                            {
                                return;
                            }

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
                            return;
                        }
                    } else {
                        if energy.consume(energy_cost) {
                            *attacker_velocity = velocity.clone();
                            return;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
