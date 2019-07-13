use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::{
    input::{Key, KeyCode::*, Mouse},
    map::Map as FovMap,
};

use crate::{
    components::*,
    resources::{
        input_action::InputAction::{self, *},
        menu::{Menu, MenuKind},
        messages::Messages,
        targeting::Targeting,
        ui::UIState,
    },
};

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    key: Write<'a, Option<Key>>,
    mouse: ReadExpect<'a, Mouse>,

    ai: WriteStorage<'a, Ai>,
    name: ReadStorage<'a, Name>,
    living: WriteStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    item: ReadStorage<'a, Item>,
    velocity: WriteStorage<'a, Velocity>,
    inventory: WriteStorage<'a, Inventory>,
    position: WriteStorage<'a, Position>,

    entity: Entities<'a>,

    menu: WriteExpect<'a, Option<Menu>>,
    ui: WriteExpect<'a, UIState>,
    action: WriteExpect<'a, InputAction>,
    messages: Write<'a, Messages>,
    fov_map: Option<ReadExpect<'a, Arc<Mutex<FovMap>>>>,
    targeting: WriteExpect<'a, Option<Targeting>>,
}

pub struct InputSystem;

impl InputSystem {
    fn handle_game_input(mut data: InputSystemData) {
        *data.action = if let Some(k) = data.key.as_ref() {
            match k {
                Key {
                    code: Enter,
                    alt: true,
                    ..
                } => ToggleFullScreen,
                Key { code: Escape, .. } => MainMenu,
                Key { code: Up, .. } => MoveNorth,
                Key { code: Right, .. } => MoveEast,
                Key { code: Down, .. } => MoveSouth,
                Key { code: Left, .. } => MoveWest,
                Key { code: Text, .. } => match k.text() {
                    "i" => OpenInventoryMenu,
                    "g" => PickUp,
                    "d" => OpenDropMenu,
                    ">" => MoveDown,
                    _ => Noop,
                },
                _ => Noop,
            }
        } else {
            Noop
        };

        *data.key = None;
    }

    fn handle_menu_input(mut data: InputSystemData) {
        let menu = data.menu.as_ref().unwrap();

        *data.action = if let Some(k) = data.key.as_ref() {
            match k {
                Key { code: Escape, .. } => DismissMenu,
                Key { code: Text, .. } => {
                    let c = k.text().chars().next();
                    if !c.map_or_else(|| false, |x| x.is_ascii_alphanumeric()) {
                        Noop
                    } else {
                        let n = c.unwrap().to_ascii_lowercase() as usize - 'a' as usize;
                        if n < menu.items.len() {
                            MenuChoice(n)
                        } else {
                            Noop
                        }
                    }
                }
                _ => Noop,
            }
        } else {
            Noop
        };

        *data.key = None;
    }

    //    fn handle_targeting_input(data: &mut InputSystemData) -> InputAction {
    //        // Cancel targeting on right mouse button and escape
    //        if data.mouse.rbutton_pressed || data.key.map_or(false, |k| k.code == Escape) {
    //            *data.targeting = None;
    //            data.messages.push("Cancelled", colors::WHITE);
    //            return InputAction::Noop;
    //        }
    //
    //        // We only want to do work if the LMB has been pressed
    //        if !data.mouse.lbutton_pressed {
    //            return InputAction::Noop;
    //        }
    //
    //        let mouse_position = Position {
    //            x: data.mouse.cx as i32,
    //            y: data.mouse.cy as i32,
    //        };
    //        let player_position: &Position = (&data.position, &data.player).join().next().unwrap().0;
    //
    //        // Only accept positions in the FOV
    //        let is_in_fov = {
    //            let fov_map_mutex = data.fov_map.as_ref().unwrap().clone();
    //            let fov_map = &*fov_map_mutex.lock().unwrap();
    //            fov_map.is_in_fov(mouse_position.x, mouse_position.y)
    //        };
    //        if !is_in_fov {
    //            return InputAction::Noop;
    //        }
    //
    //        // Apply max_range restriction, if any
    //        let on_target = {
    //            let targeting = data.targeting.as_ref().unwrap();
    //            if targeting
    //                .max_range
    //                .map_or(false, |r| r < player_position.distance_to(&mouse_position))
    //            {
    //                return InputAction::Noop;
    //            }
    //
    //            // If we're targeting monsters, ensure there's a live monster under the cursor
    //            if targeting.kind == TargetingKind::Monster
    //                && (&data.position, &data.living, &data.ai)
    //                    .join()
    //                    .find(|j| j.0 == &mouse_position && j.1.alive)
    //                    .is_none()
    //            {
    //                return Noop;
    //            }
    //
    //            // After all that, if we're still here, then the player clicked something we like
    //            match data.item.get(targeting.used_item).unwrap() {
    //                Item::Confuse => cast_confuse,
    //                Item::Fireball => cast_fireball,
    //                // TODO maybe somehow ensure in the type system we can never get here unless using a targetable thing
    //                _ => unreachable!(),
    //            }
    //        };
    //
    //        let result = on_target(&mouse_position, data);
    //
    //        let targeting = data.targeting.as_ref().unwrap();
    //        let player_action = handle_use_result(targeting.used_item, result, data);
    //
    //        // We're done targeting
    //        *data.targeting = None;
    //        player_action
    //    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if data.menu.is_some() {
            InputSystem::handle_menu_input(data);
        } else if data.targeting.is_some() {
            //            *data.action = InputSystem::handle_targeting_input(&mut data);
        } else {
            InputSystem::handle_game_input(data);
        }
    }
}
