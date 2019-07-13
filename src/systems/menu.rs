use shred_derive::SystemData;
use specs::prelude::*;

use crate::resources::ui::INVENTORY_WIDTH;
use crate::{
    components::*,
    resources::{
        input_action::InputAction,
        menu::{Menu, MenuKind},
        state::State,
    },
};

pub struct MenuSystem;

#[derive(SystemData)]
pub struct MenuSystemData<'a> {
    inventory: ReadStorage<'a, Inventory>,
    player: ReadStorage<'a, Player>,
    name: ReadStorage<'a, Name>,

    input_action: WriteExpect<'a, InputAction>,
    menu: WriteExpect<'a, Option<Menu>>,
    state: Write<'a, State>,
}

impl<'a> System<'a> for MenuSystem {
    type SystemData = MenuSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let input_action = { data.input_action.clone() };

        match input_action {
            InputAction::MainMenu => {
                *data.menu = Some(Menu {
                    items: vec![
                        "Play a new game".to_string(),
                        "Continue last game".to_string(),
                        "Quit".to_string(),
                    ],
                    header: "".to_string(),
                    width: 24,
                    kind: MenuKind::Main,
                });
                *data.state = State::MainMenu;
            }

            InputAction::OpenInventoryMenu => {
                let inventory = (&data.inventory, &data.player).join().next().unwrap().0;
                let options: Vec<String> = if inventory.0.len() == 0 {
                    vec!["Inventory is empty.".to_string()]
                } else {
                    inventory
                        .0
                        .iter()
                        .map(|item| data.name.get(*item).unwrap().0.clone())
                        .collect()
                };

                *data.menu = Some(Menu {
                    header: "Press the key next to an item to use it, escape to cancel.\n"
                        .to_string(),
                    width: INVENTORY_WIDTH,
                    items: options,
                    kind: MenuKind::Inventory,
                })
            }

            InputAction::OpenDropMenu => {
                let inventory = (&data.inventory, &data.player).join().next().unwrap().0;
                let options: Vec<String> = if inventory.0.len() == 0 {
                    vec!["Inventory is empty.".to_string()]
                } else {
                    inventory
                        .0
                        .iter()
                        .map(|item| data.name.get(*item).unwrap().0.clone())
                        .collect()
                };

                *data.menu = Some(Menu {
                    header: "Press the key next to an item to drop it, escape to cancel.\n"
                        .to_string(),
                    width: INVENTORY_WIDTH,
                    items: options,
                    kind: MenuKind::Drop,
                });
            }

            InputAction::MenuChoice(choice) => {
                if let Some(kind) = &data.menu.as_ref().map(|m| m.kind) {
                    match kind {
                        MenuKind::Main => {
                            *data.input_action = match choice {
                                0 => InputAction::NewGame,
                                1 => InputAction::LoadGame,
                                2 => InputAction::Exit,
                                _ => unreachable!(),
                            };
                            *data.menu = None;
                        }

                        MenuKind::Drop => {
                            *data.input_action = InputAction::Drop(choice);
                            *data.menu = None;
                        }

                        MenuKind::Inventory => {
                            *data.input_action = InputAction::UseFromInventory(choice);
                            *data.menu = None;
                        }
                    }
                }
            }

            InputAction::DismissMenu => {
                *data.menu = None;
            }

            _ => (),
        }
    }
}
