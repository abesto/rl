use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::*;
use crate::PlayerAction;

#[derive(SystemData)]
pub struct AISystemData<'a> {
    player: ReadStorage<'a, Player>,
    action: ReadExpect<'a, PlayerAction>,
    alive: ReadStorage<'a, Alive>,
    name: ReadStorage<'a, Name>,
}

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = AISystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        // Only run if the player is alive
        for (_, alive) in (&data.player, &data.alive).join() {
            if !alive.0 {
                return;
            }
        }
        // Only run if the player took a turn
        if *data.action == PlayerAction::DidntTakeTurn {
            return;
        }
        // Run AI for anything that's alive and is not the player
        for (alive, name, _) in (&data.alive, &data.name, !&data.player).join() {
            if alive.0 {
                println!("The {} growls!", name.0);
            }
        }
    }
}
