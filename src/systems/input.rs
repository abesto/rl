use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;

use crate::components::velocity::Heading::*;
use crate::components::*;
use crate::resources::messages::Messages;
use crate::resources::ui::UIState;
use crate::PlayerAction;
use crate::PlayerAction::*;
use shred::PanicHandler;

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    key: Write<'a, Option<Key>>,

    name: ReadStorage<'a, Name>,
    living: ReadStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    item: ReadStorage<'a, Item>,
    velocity: WriteStorage<'a, Velocity>,
    inventory: WriteStorage<'a, Inventory>,
    position: WriteStorage<'a, Position>,

    entity: Entities<'a>,

    ui: WriteExpect<'a, UIState>,
    action: WriteExpect<'a, PlayerAction>,
    messages: WriteExpect<'a, Messages>,
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if let Some((vel, living, inventory, _)) = (
            &mut data.velocity,
            &data.living,
            &mut data.inventory,
            &data.player,
        )
            .join()
            .next()
        {
            *data.action = if let Some(k) = *data.key {
                match (k, living.alive) {
                    (
                        Key {
                            code: Enter,
                            alt: true,
                            ..
                        },
                        _,
                    ) => {
                        // Alt+Enter: toggle fullscreen
                        data.ui.config.fullscreen = !data.ui.config.fullscreen;
                        DidntTakeTurn
                    }
                    (Key { code: Escape, .. }, _) => Exit,
                    (Key { code: Up, .. }, true) => {
                        *vel = Velocity::unit(North);
                        TookTurn
                    }
                    (Key { code: Right, .. }, true) => {
                        *vel = Velocity::unit(East);
                        TookTurn
                    }
                    (Key { code: Down, .. }, true) => {
                        *vel = Velocity::unit(South);
                        TookTurn
                    }
                    (Key { code: Left, .. }, true) => {
                        *vel = Velocity::unit(West);
                        TookTurn
                    }
                    (
                        Key {
                            code: Char,
                            printable: 'g',
                            ..
                        },
                        true,
                    ) => {
                        // pick up an item
                        let (player_pos, _) = (&data.position, &data.player).join().next().unwrap();
                        if let Some((item, name, _, _)) =
                            (&data.entity, &data.name, &data.position, &data.item)
                                .join()
                                .find(|j| j.2 == player_pos)
                        {
                            pick_item_up(item, name, data.position, inventory, data.messages);
                        }
                        DidntTakeTurn
                    }
                    _ => DidntTakeTurn,
                }
            } else {
                DidntTakeTurn
            };

            *data.key = None;
        }
    }
}

fn pick_item_up(
    item: Entity,
    item_name: &Name,
    mut position: WriteStorage<Position>,
    inventory: &mut Inventory,
    mut messages: Write<Messages, PanicHandler>,
) {
    if inventory.0.len() >= 26 {
        messages.push(
            format!("Your inventory is full, cannot pick up {}.", item_name.0),
            colors::RED,
        );
    } else {
        position.remove(item);
        messages.push(format!("You picked up a {}!", item_name.0), colors::GREEN);
        inventory.0.push(item);
    }
}
