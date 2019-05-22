use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::map::Map as FovMap;

use crate::components::*;
use crate::PlayerAction;

#[derive(SystemData)]
pub struct AISystemData<'a> {
    ai: ReadStorage<'a, Ai>,
    living: ReadStorage<'a, Living>,
    player: ReadStorage<'a, Player>,

    position: ReadStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,

    action: ReadExpect<'a, PlayerAction>,
    fov_map: ReadExpect<'a, Arc<Mutex<FovMap>>>,
}

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = AISystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let fov_map_mutex = data.fov_map.clone();
        let fov_map = &*fov_map_mutex.lock().unwrap();

        // Used so that monsters don't step on each others toes
        let mut will_move_to: HashSet<Position> = HashSet::new();

        // Only run if the player took a turn
        if *data.action == PlayerAction::DidntTakeTurn {
            return;
        }

        // Let's find the player...
        let (_, player_living, player_position) = (&data.player, &data.living, &data.position)
            .join()
            .next()
            .unwrap();

        // Only run if the player is living
        if !player_living.alive {
            return;
        }

        // Run AI for anything that's AI-controlled and living
        for (_, monster_position, monster_velocity, _) in
            (&data.living, &data.position, &mut data.velocity, &data.ai)
                .join()
                .filter(|j| j.0.alive && fov_map.is_in_fov(j.1.x, j.1.y))
        {
            let velocity = monster_position.move_towards(player_position);
            let candidate = monster_position + &velocity;
            if player_position == &candidate || !will_move_to.contains(&candidate) {
                *monster_velocity = velocity;
                will_move_to.insert(candidate);
            }
        }
    }
}
