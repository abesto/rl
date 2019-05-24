use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;

use crate::components::*;
use crate::resources::messages::Messages;

pub struct PlayerDeathSystem;

#[derive(SystemData)]
pub struct PlayerDeathSystemData<'a> {
    living: WriteStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    visual: WriteStorage<'a, Visual>,

    messages: WriteExpect<'a, Messages>,
}

impl<'a> System<'a> for PlayerDeathSystem {
    type SystemData = PlayerDeathSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        if let Some((mut living, _, mut visual)) =
            (&mut data.living, &data.player, &mut data.visual)
                .join()
                .find(|j| j.0.alive && j.0.hp <= 0)
        {
            // the game ended!
            data.messages.push("You died!", colors::RED);
            living.alive = false;

            // for added effect, transform the player into a corpse!
            visual.char = '%';
            visual.color = colors::DARK_RED;
        }
    }
}
