use std::sync::{Arc, Mutex};

use shred::PanicHandler;
use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use tcod::map::Map as FovMap;

use crate::components::velocity::Heading::*;
use crate::components::*;
use crate::resources::menu::{Menu, MenuKind};
use crate::resources::messages::Messages;
use crate::resources::ui::{UIState, INVENTORY_WIDTH};
use crate::PlayerAction;
use crate::PlayerAction::*;

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    key: Write<'a, Option<Key>>,

    ai: ReadStorage<'a, Ai>,
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
    action: WriteExpect<'a, PlayerAction>,
    messages: WriteExpect<'a, Messages>,
    fov_map: ReadExpect<'a, Arc<Mutex<FovMap>>>,
}

pub struct InputSystem;

// TODO: refactor such that this system populates an action queue
// and other systems implement the actions themselves. One action is EndTurn.

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
            *data.action = if let Some(k) = data.key.as_ref() {
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
                            )
                        } else {
                            DidntTakeTurn
                        }
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
                MenuKind::Inventory => use_item_from_inventory(n, &mut data),
            }
        }

        // If the player made a valid choice, or pressed escape, dismiss the menu
        if choice.is_some() || data.key.map_or(false, |k| k.code == Escape) {
            *data.menu = None;
        }
        *data.key = None;
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
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
) -> PlayerAction {
    if inventory.0.len() >= 26 {
        messages.push(
            format!("Your inventory is full, cannot pick up {}.", item_name.0),
            colors::RED,
        );
        DidntTakeTurn
    } else {
        position.remove(item);
        messages.push(format!("You picked up a {}!", item_name.0), colors::GREEN);
        inventory.0.push(item);
        TookTurn
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
        header: "Press the key next to an item to use it, escape to cancel.\n".to_string(),
        width: INVENTORY_WIDTH,
        items: options,
        kind: MenuKind::Inventory,
    }
}

fn use_item_from_inventory(inventory_id: usize, data: &mut InputSystemData) -> PlayerAction {
    // We have a bunch of separate borrows of data here to get the inventory.
    // Would be good to get rid of that; one way might be passing subsets of the
    // input system to on_use functions, but generalization will make that a pain.
    let opt_item = {
        let (inventory, _) = (&mut data.inventory, &data.player).join().next().unwrap();
        if let Some(entity) = inventory.0.get(inventory_id) {
            data.item.get(*entity)
        } else {
            return DidntTakeTurn;
        }
    };
    if let Some(item) = opt_item {
        let on_use = match item {
            Item::Heal => cast_heal,
            Item::Lightning => cast_lightning,
        };
        match on_use(inventory_id, data) {
            UseResult::UsedUp => {
                // destroy after use, unless it was cancelled for some reason
                let (inventory, _) = (&mut data.inventory, &data.player).join().next().unwrap();
                inventory.0.remove(inventory_id);
                TookTurn
            }
            UseResult::Cancelled => {
                data.messages.push("Cancelled", colors::WHITE);
                DidntTakeTurn
            }
        }
    } else {
        let (inventory, _) = (&mut data.inventory, &data.player).join().next().unwrap();
        data.messages.push(
            format!(
                "The {} cannot be used.",
                data.name.get(inventory.0[inventory_id]).unwrap().0
            ),
            colors::WHITE,
        );
        DidntTakeTurn
    }
}

enum UseResult {
    UsedUp,
    Cancelled,
}

const HEAL_AMOUNT: i32 = 4;

fn cast_heal(_inventory_id: usize, data: &mut InputSystemData) -> UseResult {
    // heal the player
    if let Some((living, _)) = (&mut data.living, &data.player).join().next() {
        if living.hp == living.max_hp {
            data.messages
                .push("You are already at full health.", colors::RED);
            UseResult::Cancelled
        } else {
            data.messages
                .push("Your wounds start to feel better!", colors::LIGHT_VIOLET);
            heal(living, HEAL_AMOUNT);
            UseResult::UsedUp
        }
    } else {
        UseResult::Cancelled
    }
}

/// heal by the given amount, without going over the maximum
pub fn heal(living: &mut Living, amount: i32) {
    living.hp = living.max_hp.min(living.hp + amount);
}

const LIGHTNING_RANGE: i32 = 5;
const LIGHTNING_DAMAGE: i32 = 20;

fn cast_lightning(_inventory_id: usize, data: &mut InputSystemData) -> UseResult {
    // find closest enemy (inside a maximum range and damage it)
    let monster = closest_monster(LIGHTNING_RANGE, data);
    if let Some(monster) = monster {
        // zap it!
        data.messages.push(
            format!(
                "A lightning bolt strikes the {} with a loud thunder! \
                 The damage is {} hit points.",
                data.name.get(monster).unwrap().0,
                LIGHTNING_DAMAGE
            ),
            colors::LIGHT_BLUE,
        );
        data.living.get_mut(monster).unwrap().hp -= LIGHTNING_DAMAGE;
        UseResult::UsedUp
    } else {
        // no enemy found within maximum range
        data.messages
            .push("No enemy is close enough to strike.", colors::RED);
        UseResult::Cancelled
    }
}

/// find closest enemy, up to a maximum range, and in the player's FOV
fn closest_monster(max_range: i32, data: &mut InputSystemData) -> Option<Entity> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32; // start with (slightly more than) maximum range

    let player_pos = (&data.position, &data.player).join().next().unwrap().0;

    let fov_map_mutex = data.fov_map.clone();
    let fov_map = &*fov_map_mutex.lock().unwrap();

    for (entity, pos, _, _) in (&data.entity, &data.position, &data.living, &data.ai)
        .join()
        .filter(|j| j.2.alive && fov_map.is_in_fov(j.1.x, j.1.y))
    {
        // calculate distance between this object and the player
        let dist = player_pos.distance_to(pos);
        if dist < closest_dist {
            // it's closer, so remember it
            closest_enemy = Some(entity);
            closest_dist = dist;
        }
    }
    closest_enemy
}
