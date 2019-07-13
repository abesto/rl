use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::*;
use crate::resources::messages::Messages;
use tcod::colors;

pub struct DropSystem;

#[derive(SystemData)]
pub struct DropSystemData<'a> {
    inventory: WriteStorage<'a, Inventory>,
    position: WriteStorage<'a, Position>,
    player: ReadStorage<'a, Player>,
    action: ReadStorage<'a, Action>,
    name: ReadStorage<'a, Name>,
    energy: WriteStorage<'a, Energy>,

    messages: Write<'a, Messages>,

    entity: Entities<'a>,
}

impl<'a> System<'a> for DropSystem {
    type SystemData = DropSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (inventory, action, energy, actor) in (
            &mut data.inventory,
            &data.action,
            &mut data.energy,
            &data.entity,
        )
            .join()
            .map(|j| (j.0, j.1, j.2, j.3))
        {
            match *action {
                Action::Drop { inventory_index } => {
                    if let Some(&entity) = { inventory.0.get(inventory_index) } {
                        if energy.consume(action.energy_cost()) {
                            let position = { data.position.get(actor).unwrap().clone() };
                            data.position.insert(entity, position).unwrap();
                            inventory.0.remove(inventory_index);
                            data.messages.push(
                                format!("You dropped a {}.", data.name.get(entity).unwrap().0),
                                colors::YELLOW,
                            );
                        }
                    }
                }
                _ => (),
            };
        }
    }
}
