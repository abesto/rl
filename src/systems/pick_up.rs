use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;

use crate::{components::*, resources::messages::Messages};

pub struct PickUpSystem;

#[derive(SystemData)]
pub struct PickUpSystemData<'a> {
    position: WriteStorage<'a, Position>,
    entity: Entities<'a>,
    inventory: WriteStorage<'a, Inventory>,
    action: ReadStorage<'a, Action>,
    energy: WriteStorage<'a, Energy>,

    name: ReadStorage<'a, Name>,
    item: ReadStorage<'a, Item>,

    messages: Write<'a, Messages>,
}

impl<'a> System<'a> for PickUpSystem {
    type SystemData = PickUpSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (actor, inventory, action, energy) in (
            &data.entity,
            &mut data.inventory,
            &data.action,
            &mut data.energy,
        )
            .join()
            .filter(|j| *j.2 == Action::PickUp)
        {
            let position = data.position.get(actor);
            if position.is_none() {
                continue;
            }
            let position = position.unwrap();
            if let Some((item, name, _, _)) = (&data.entity, &data.name, &data.position, &data.item)
                .join()
                .find(|j| j.2 == position)
            {
                if inventory.0.len() >= 26 {
                    // Note, if monsters ever learn to pick things up, this needs to change.
                    // Ideally into some perception system.
                    data.messages.push(
                        format!("Your inventory is full, cannot pick up {}.", name.0),
                        colors::RED,
                    );
                } else if energy.consume(action.energy_cost()) {
                    data.position.remove(item);
                    data.messages
                        .push(format!("You picked up a {}!", name.0), colors::GREEN);
                    inventory.0.push(item);
                }
            }
        }
    }
}
