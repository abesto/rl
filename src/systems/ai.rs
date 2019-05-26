use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use shred_derive::SystemData;
use specs::prelude::*;
use tcod::{colors, map::Map as FovMap};

use crate::{
    components::{velocity::Heading, *},
    resources::{messages::Messages, state::State},
    PlayerAction,
};

#[derive(SystemData)]
pub struct AISystemData<'a> {
    ai: WriteStorage<'a, Ai>,
    living: ReadStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    name: ReadStorage<'a, Name>,
    entity: Entities<'a>,

    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    state: ReadExpect<'a, State>,
    action: ReadExpect<'a, PlayerAction>,
    fov_map: Option<ReadExpect<'a, Arc<Mutex<FovMap>>>>,
    messages: WriteExpect<'a, Messages>,
}

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = AISystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // AI only runs when the game is on
        if *data.state != State::Game {
            return;
        }

        // Only run if the player took a turn
        if *data.action == PlayerAction::DidntTakeTurn {
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

        // Used so that monsters don't step on each others toes
        let mut will_move_to: HashSet<Position> = HashSet::new();

        // Select the entities we'll want to apply AI logic to
        let monsters: Vec<Entity> = {
            let fov_map_mutex = data.fov_map.as_ref().unwrap().clone();
            let fov_map = &*fov_map_mutex.lock().unwrap();
            (
                &data.living,
                &data.position,
                &data.velocity,
                &data.ai,
                &data.entity,
            )
                .join()
                .filter(|j| j.0.alive && fov_map.is_in_fov(j.1.x, j.1.y))
                .map(|j| j.4)
                .collect()
        };

        for monster in monsters {
            // Decide where we want to go
            let velocity = run_ai(monster, &mut data);

            // Make sure we don't step on each others toes
            let candidate = data.position.get(monster).unwrap() + &velocity;
            let is_attack = (&data.position, &data.living, &data.player)
                .join()
                .find(|j| j.0 == &candidate && j.1.alive)
                .is_some();
            let is_friendly_attack = (&data.position, &data.living, &data.ai)
                .join()
                .find(|j| j.0 == &candidate && j.1.alive)
                .is_some();
            if is_attack || (!is_friendly_attack && !will_move_to.contains(&candidate)) {
                *data.velocity.get_mut(monster).unwrap() = velocity;
                will_move_to.insert(candidate);
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

fn run_ai(entity: Entity, data: &mut AISystemData) -> Velocity {
    let ai = data.ai.get(entity).unwrap();
    match ai {
        Ai::Basic => basic_ai(entity, data),
        Ai::Confused { .. } => confused_ai(entity, data),
    }
}

fn basic_ai(entity: Entity, data: &mut AISystemData) -> Velocity {
    let (player_pos, _) = (&data.position, &data.player).join().next().unwrap();
    let monster_pos = data.position.get(entity).unwrap();
    monster_pos.move_towards(player_pos)
}

fn confused_ai(entity: Entity, data: &mut AISystemData) -> Velocity {
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
                Velocity {
                    magnitude: 1,
                    heading: rand::thread_rng().gen(),
                }
            }
        }
        _ => unreachable!(),
    }
}
