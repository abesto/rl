use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;

use crate::components::velocity::Heading::*;
use crate::components::*;
use crate::resources::menu::{Menu, MenuKind};
use crate::resources::messages::Messages;
use crate::resources::ui::{UIState, INVENTORY_WIDTH};
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

    menu: WriteExpect<'a, Option<Menu>>,
    ui: WriteExpect<'a, UIState>,
    action: WriteExpect<'a, PlayerAction>,
    messages: WriteExpect<'a, Messages>,
}

pub struct InputSystem;

impl InputSystem {
    fn handle_game_input(mut data: InputSystemData) {
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
                            printable: 'i',
                            ..
                        },
                        true,
                    ) => {
                        // how a menu with each item of the inventory as an option
                        *data.menu = Some(inventory_menu(&inventory, data.name));
                        DidntTakeTurn
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
                            pick_item_up(
                                item,
                                name,
                                &mut data.position,
                                inventory,
                                &mut data.messages,
                            );
                        }
                        DidntTakeTurn
                    }
                    _ => DidntTakeTurn,
                }
            } else {
                DidntTakeTurn
            };
        }

        *data.key = None;
    }

    fn handle_menu_input(mut data: InputSystemData) {
        if let Some(Key { code: Char, .. }) = *data.key {
            *data.menu = None;
        }
        *data.key = None;
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if data.menu.is_some() {
            InputSystem::handle_menu_input(data);
        } else {
            InputSystem::handle_game_input(data);
        }
    }
}

fn pick_item_up(
    item: Entity,
    item_name: &Name,
    position: &mut WriteStorage<Position>,
    inventory: &mut Inventory,
    messages: &mut Write<Messages, PanicHandler>,
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

fn inventory_menu(inventory: &Inventory, name: ReadStorage<Name>) -> Menu {
    let options: Vec<String> = if inventory.0.len() == 0 {
        vec!["Inventory is empty.".to_string()]
    } else {
        inventory
            .0
            .iter()
            .map(|item| name.get(*item).unwrap().0.clone())
            .collect()
    };

    Menu {
        header: "Press the key next to an item to use it, or any other to cancel.\n".to_string(),
        width: INVENTORY_WIDTH,
        items: options,
        kind: MenuKind::Inventory,
    }
}
