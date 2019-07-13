use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::*;

pub struct TimeSystem;

#[derive(SystemData)]
pub struct TimeSystemData<'a> {
    energy: WriteStorage<'a, Energy>,
    player: ReadStorage<'a, Player>,
    living: ReadStorage<'a, Living>,
}

impl<'a> System<'a> for TimeSystem {
    type SystemData = TimeSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // If the player has enough energy to act, stop ticking until they do
        if (&data.energy, &data.player)
            .join()
            .next()
            .map(|j| j.0.can_act())
            .unwrap_or(true)
        {
            return;
        }

        // If the player is dead, don't tick
        if (&data.player, &data.living)
            .join()
            .next()
            .map(|j| !j.1.alive)
            .unwrap_or(true)
        {
            return;
        }

        // Tick!
        for energy in (&mut data.energy).join() {
            energy.gain(1);
        }
    }
}
