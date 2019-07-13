use std::sync::{Arc, Mutex};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use shred_derive::SystemData;
use specs::prelude::*;
use tcod::{colors, map::Map as FovMap};

use crate::{
    components::{velocity::Heading, *},
    resources::{input_action::InputAction, messages::Messages, state::State},
};

#[derive(SystemData)]
pub struct AISystemData<'a> {
    ai: WriteStorage<'a, Ai>,
    living: ReadStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    name: ReadStorage<'a, Name>,
    entity: Entities<'a>,
    action: WriteStorage<'a, Action>,
    energy: ReadStorage<'a, Energy>,

    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    input_action: Write<'a, InputAction>,
    state: ReadExpect<'a, State>,
    fov_map: Option<ReadExpect<'a, Arc<Mutex<FovMap>>>>,
    messages: Write<'a, Messages>,
}

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = AISystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // AI only runs when the game is on
        if *data.state != State::Game {
            return;
        }

        // Only run if the player is living
        if !(&data.player, &data.living)
            .join()
            .next()
            .map_or(false, |j| j.1.alive)
        {
            return;
        }

        // Select the entities we'll want to apply AI logic to
        let monsters: Vec<(Entity, bool)> = {
            let fov_map_mutex = data.fov_map.as_ref().unwrap().clone();
            let fov_map = &*fov_map_mutex.lock().unwrap();
            (&data.living, &data.position, &data.ai, &data.entity)
                .join()
                .filter(|j| j.0.alive)
                .map(|j| (j.3, fov_map.is_in_fov(j.1.x, j.1.y)))
                .collect()
        };

        // And run that AI
        for (monster, visible) in monsters {
            *data.action.get_mut(monster).unwrap() = if visible {
                run_ai(monster, &mut data)
            } else {
                Action::Skip { ticks: 1 }
            }
        }
    }
}

impl Distribution<Heading> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Heading {
        match rng.gen_range(0, 3) {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => unreachable!(),
        }
    }
}

fn run_ai(entity: Entity, data: &mut AISystemData) -> Action {
    let ai = data.ai.get(entity).unwrap();
    match ai {
        Ai::Basic => basic_ai(entity, data),
        Ai::Confused { .. } => confused_ai(entity, data),
        Ai::Player => player_ai(data),
    }
}

fn basic_ai(entity: Entity, data: &mut AISystemData) -> Action {
    let (player_pos, _) = (&data.position, &data.player).join().next().unwrap();
    let monster_pos = data.position.get(entity).unwrap();
    Action::MoveOrMelee {
        velocity: monster_pos.move_towards(player_pos),
        attack_monsters: false,
        attack_player: true,
    }
}

fn confused_ai(entity: Entity, data: &mut AISystemData) -> Action {
    let ai = data.ai.get_mut(entity).unwrap();
    match ai {
        Ai::Confused {
            ref mut num_turns,
            previous_ai,
        } => {
            *num_turns -= 1;
            if *num_turns == 0 {
                *ai = *previous_ai.clone();
                data.messages.push(
                    format!(
                        "The {} is no longer confused!",
                        data.name.get(entity).unwrap().0
                    ),
                    colors::RED,
                );
                run_ai(entity, data)
            } else {
                Action::MoveOrMelee {
                    velocity: Velocity {
                        magnitude: 1,
                        heading: rand::thread_rng().gen(),
                    },
                    attack_monsters: true,
                    attack_player: true,
                }
            }
        }
        _ => unreachable!(),
    }
}

fn player_move_or_melee(heading: Heading) -> Action {
    Action::MoveOrMelee {
        velocity: Velocity::from(heading),
        attack_monsters: true,
        attack_player: false,
    }
}

fn player_ai(data: &mut AISystemData) -> Action {
    use crate::resources::input_action::InputAction::*;
    let action = match *data.input_action {
        MoveNorth => player_move_or_melee(Heading::North),
        MoveEast => player_move_or_melee(Heading::East),
        MoveSouth => player_move_or_melee(Heading::South),
        MoveWest => player_move_or_melee(Heading::West),
        _ => Action::WaitForInput,
    };
    *data.input_action = InputAction::Noop;
    action
}
