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

        let choice: Option<usize> = if let Some(Key {
            code: Char,
            printable,
            ..
        }) = data.key.as_ref()
        {
            if !printable.is_ascii_alphanumeric() {
                None
            } else {
                let n = printable.to_ascii_lowercase() as usize - 'a' as usize;
                if n <= menu.items.len() {
                    Some(n)
                } else {
                    None
                }
            }
        } else {
            None
        };

        if let Some(n) = choice {
            *data.action = match menu.kind {
                //                MenuKind::Inventory => use_item_from_inventory(n, &mut data),
                //                MenuKind::Drop => drop_item(n, &mut data),
                MenuKind::Inventory => InputAction::Noop,
                MenuKind::Drop => InputAction::Noop,
                MenuKind::Main => handle_main_menu_choice(n, &mut data),
            }
        }

        // If the player made a valid choice, or pressed escape, dismiss the menu
        let escape_pressed = data.key.map_or(false, |k| k.code == Escape);
        let is_main_menu = data.menu.as_ref().unwrap().kind == MenuKind::Main;
        if choice.is_some() || (escape_pressed && !is_main_menu) {
            *data.menu = None;
        }
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

//fn pick_item_up(
//    item: Entity,
//    item_name: &Name,
//    position: &mut WriteStorage<Position>,
//    inventory: &mut Inventory,
//    messages: &mut Write<Messages, PanicHandler>,
//) -> InputAction {
//    if inventory.0.len() >= 26 {
//        messages.push(
//            format!("Your inventory is full, cannot pick up {}.", item_name.0),
//            colors::RED,
//        );
//        Action::Noop
//    } else {
//        position.remove(item);
//        messages.push(format!("You picked up a {}!", item_name.0), colors::GREEN);
//        inventory.0.push(item);
//        TookTurn
//    }
//}
//
//fn inventory_menu(inventory: &Inventory, name: ReadStorage<Name>) -> Menu {
//    let options: Vec<String> = if inventory.0.len() == 0 {
//        vec!["Inventory is empty.".to_string()]
//    } else {
//        inventory
//            .0
//            .iter()
//            .map(|item| name.get(*item).unwrap().0.clone())
//            .collect()
//    };
//
//    Menu {
//        header: "Press the key next to an item to use it, escape to cancel.\n".to_string(),
//        width: INVENTORY_WIDTH,
//        items: options,
//        kind: MenuKind::Inventory,
//    }
//}
//
//fn use_item_from_inventory(inventory_id: usize, data: &mut InputSystemData) -> InputAction {
//    let (&entity, opt_item) = {
//        let (inventory, _) = (&mut data.inventory, &data.player).join().next().unwrap();
//        if let Some(entity) = inventory.0.get(inventory_id) {
//            (entity, data.item.get(*entity))
//        } else {
//            return DidntTakeTurn;
//        }
//    };
//    if let Some(item) = opt_item {
//        let on_use = match item {
//            Item::Heal => cast_heal,
//            Item::Lightning => cast_lightning,
//            Item::Confuse => target_confuse,
//            Item::Fireball => target_fireball,
//        };
//        let result = on_use(entity, data);
//        handle_use_result(entity, result, data)
//    } else {
//        let (inventory, _) = (&mut data.inventory, &data.player).join().next().unwrap();
//        data.messages.push(
//            format!(
//                "The {} cannot be used.",
//                data.name.get(inventory.0[inventory_id]).unwrap().0
//            ),
//            colors::WHITE,
//        );
//        DidntTakeTurn
//    }
//}
//
//fn drop_menu(inventory: &Inventory, name: ReadStorage<Name>) -> Menu {
//    let options: Vec<String> = if inventory.0.len() == 0 {
//        vec!["Inventory is empty.".to_string()]
//    } else {
//        inventory
//            .0
//            .iter()
//            .map(|item| name.get(*item).unwrap().0.clone())
//            .collect()
//    };
//
//    Menu {
//        header: "Press the key next to an item to drop it, escape to cancel.\n".to_string(),
//        width: INVENTORY_WIDTH,
//        items: options,
//        kind: MenuKind::Drop,
//    }
//}
//
//fn drop_item(inventory_id: usize, data: &mut InputSystemData) -> InputAction {
//    let (&entity, position) = {
//        let (inventory, position, _) = (&mut data.inventory, &data.position, &data.player)
//            .join()
//            .next()
//            .unwrap();
//        if let Some(entity) = inventory.0.get(inventory_id) {
//            (entity, position.clone())
//        } else {
//            return DidntTakeTurn;
//        }
//    };
//    data.position.insert(entity, position).unwrap();
//    let (inventory, _) = (&mut data.inventory, &data.player).join().next().unwrap();
//    inventory.0.remove(inventory_id);
//    data.messages.push(
//        format!("You dropped a {}.", data.name.get(entity).unwrap().0),
//        colors::YELLOW,
//    );
//    TookTurn
//}
//
//fn handle_use_result(entity: Entity, result: UseResult, data: &mut InputSystemData) -> InputAction {
//    match result {
//        UseResult::UsedUp => {
//            // destroy after use, unless it was cancelled for some reason
//            let (inventory, _) = (&mut data.inventory, &data.player).join().next().unwrap();
//            inventory.0.retain(|&x| x != entity);
//            TookTurn
//        }
//        UseResult::Cancelled => {
//            data.messages.push("Cancelled", colors::WHITE);
//            DidntTakeTurn
//        }
//        UseResult::Targeting => DidntTakeTurn,
//    }
//}
//
//enum UseResult {
//    UsedUp,
//    Cancelled,
//    Targeting,
//}
//
//const HEAL_AMOUNT: i32 = 4;
//
//fn cast_heal(_entity: Entity, data: &mut InputSystemData) -> UseResult {
//    // heal the player
//    if let Some((living, _)) = (&mut data.living, &data.player).join().next() {
//        if living.hp == living.max_hp {
//            data.messages
//                .push("You are already at full health.", colors::RED);
//            UseResult::Cancelled
//        } else {
//            data.messages
//                .push("Your wounds start to feel better!", colors::LIGHT_VIOLET);
//            heal(living, HEAL_AMOUNT);
//            UseResult::UsedUp
//        }
//    } else {
//        UseResult::Cancelled
//    }
//}
//
///// heal by the given amount, without going over the maximum
//pub fn heal(living: &mut Living, amount: i32) {
//    living.hp = living.max_hp.min(living.hp + amount);
//}
//
//const LIGHTNING_RANGE: i32 = 5;
//const LIGHTNING_DAMAGE: i32 = 20;
//
//fn cast_lightning(_entity: Entity, data: &mut InputSystemData) -> UseResult {
//    // find closest enemy (inside a maximum range and damage it)
//    let monster = closest_monster(LIGHTNING_RANGE, data);
//    if let Some(monster) = monster {
//        // zap it!
//        data.messages.push(
//            format!(
//                "A lightning bolt strikes the {} with a loud thunder! \
//                 The damage is {} hit points.",
//                data.name.get(monster).unwrap().0,
//                LIGHTNING_DAMAGE
//            ),
//            colors::LIGHT_BLUE,
//        );
//        data.living.get_mut(monster).unwrap().hp -= LIGHTNING_DAMAGE;
//        UseResult::UsedUp
//    } else {
//        // no enemy found within maximum range
//        data.messages
//            .push("No enemy is close enough to strike.", colors::RED);
//        UseResult::Cancelled
//    }
//}
//
//const CONFUSE_RANGE: f32 = 8.0;
//const CONFUSE_NUM_TURNS: i32 = 10;
//
//fn target_confuse(entity: Entity, data: &mut InputSystemData) -> UseResult {
//    *data.targeting = Some(Targeting {
//        used_item: entity,
//        kind: TargetingKind::Monster,
//        max_range: Some(CONFUSE_RANGE),
//    });
//    data.messages.push(
//        "Left-click an enemy to confuse it, or right-click to cancel.",
//        colors::CYAN,
//    );
//    UseResult::Targeting
//}
//
//fn cast_confuse(position: &Position, data: &mut InputSystemData) -> UseResult {
//    let monster = (&data.position, &data.ai, &data.living, &data.entity)
//        .join()
//        .find(|j| j.0 == position && j.2.alive)
//        .unwrap()
//        .3;
//    let old_ai = data.ai.get(monster).map(Clone::clone).unwrap_or(Ai::Basic);
//    data.ai
//        .insert(
//            monster,
//            Ai::Confused {
//                previous_ai: Box::new(old_ai),
//                num_turns: CONFUSE_NUM_TURNS,
//            },
//        )
//        .unwrap();
//    data.messages.push(
//        format!(
//            "The eyes of {} look vacant, as he starts to stumble around!",
//            &data.name.get(monster).unwrap().0
//        ),
//        colors::LIGHT_GREEN,
//    );
//    UseResult::UsedUp
//}
//
//const FIREBALL_RADIUS: f32 = 3.0;
//const FIREBALL_DAMAGE: i32 = 12;
//
//fn target_fireball(entity: Entity, data: &mut InputSystemData) -> UseResult {
//    *data.targeting = Some(Targeting {
//        used_item: entity,
//        kind: TargetingKind::Tile,
//        max_range: None,
//    });
//    data.messages.push(
//        "Left-click a target tile for the fireball, or right-click to cancel.",
//        colors::CYAN,
//    );
//    UseResult::Targeting
//}
//
//fn cast_fireball(position: &Position, data: &mut InputSystemData) -> UseResult {
//    data.messages.push(
//        format!(
//            "The fireball explodes, burning everything within {} tiles!",
//            FIREBALL_RADIUS
//        ),
//        colors::ORANGE,
//    );
//    for (target_position, living, name) in (&data.position, &mut data.living, &data.name).join() {
//        if position.distance_to(&target_position) <= FIREBALL_RADIUS && living.alive {
//            data.messages.push(
//                format!(
//                    "The {} gets burned for {} hit points.",
//                    name.0, FIREBALL_DAMAGE
//                ),
//                colors::ORANGE,
//            );
//            living.hp -= FIREBALL_DAMAGE;
//        }
//    }
//    UsedUp
//}
//
///// find closest enemy, up to a maximum range, and in the player's FOV
//fn closest_monster(max_range: i32, data: &mut InputSystemData) -> Option<Entity> {
//    let mut closest_enemy = None;
//    let mut closest_dist = (max_range + 1) as f32; // start with (slightly more than) maximum range
//
//    let player_pos = (&data.position, &data.player).join().next().unwrap().0;
//
//    let fov_map_mutex = data.fov_map.as_ref().unwrap().clone();
//    let fov_map = &*fov_map_mutex.lock().unwrap();
//
//    for (entity, pos, _, _) in (&data.entity, &data.position, &data.living, &data.ai)
//        .join()
//        .filter(|j| j.2.alive && fov_map.is_in_fov(j.1.x, j.1.y))
//    {
//        // calculate distance between this object and the player
//        let dist = player_pos.distance_to(pos);
//        if dist < closest_dist {
//            // it's closer, so remember it
//            closest_enemy = Some(entity);
//            closest_dist = dist;
//        }
//    }
//    closest_enemy
//}

fn handle_main_menu_choice(n: usize, _data: &mut InputSystemData) -> InputAction {
    match n {
        0 => NewGame,
        1 => LoadGame,
        2 => Exit,
        _ => unreachable!(),
    }
}
