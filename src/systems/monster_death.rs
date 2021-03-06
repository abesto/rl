use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;

use crate::{
    components::*,
    resources::{messages::Messages, state::State},
};

pub struct MonsterDeathSystem;

#[derive(SystemData)]
pub struct MonsterDeathSystemData<'a> {
    living: WriteStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    name: WriteStorage<'a, Name>,

    visual: WriteStorage<'a, Visual>,
    ai: WriteStorage<'a, Ai>,
    collider: WriteStorage<'a, Collider>,
    power: WriteStorage<'a, Power>,
    energy: WriteStorage<'a, Energy>,
    action: WriteStorage<'a, Action>,

    state: ReadExpect<'a, State>,
    messages: Write<'a, Messages>,
    entity: Entities<'a>,
}

impl<'a> System<'a> for MonsterDeathSystem {
    type SystemData = MonsterDeathSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.state != State::Game {
            return;
        }

        if let Some((mut living, mut name, _, mut visual, entity)) = (
            &mut data.living,
            &mut data.name,
            !&data.player,
            &mut data.visual,
            &data.entity,
        )
            .join()
            .find(|j| j.0.alive && j.0.hp <= 0)
        {
            // transform it into a nasty corpse! it doesn't block, can't be
            // attacked and doesn't move
            data.messages
                .push(format!("{} is dead!", name.0), colors::ORANGE);

            // You are now dead
            living.alive = false;

            // Horrible to behold
            visual.char = '%';
            visual.color = colors::DARK_RED;

            data.ai.remove(entity); // Stripped of your intelligence
            data.power.remove(entity); // Stripped of your power
            data.collider.remove(entity); // Stripped of your very essence
            data.action.remove(entity); // No last words for you
            data.energy.remove(entity); // Rest shall grant you no reprieve
            name.0 = format!("remains of {}", name.0); // Even your name shall be forgotten
        }
    }
}
