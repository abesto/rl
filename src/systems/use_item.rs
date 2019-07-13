use std::sync::{Arc, Mutex};

use shred::PanicHandler;
use shred_derive::SystemData;
use specs::prelude::*;
use tcod::map::Map as FovMap;

use crate::components::*;
use crate::resources::{
    input_action::InputAction,
    messages::Messages,
    targeting::{Targeting, TargetingKind},
};
use tcod::colors;

pub struct UseItemSystem;

#[derive(SystemData)]
pub struct UseItemSystemData<'a> {
    inventory: WriteStorage<'a, Inventory>,
    position: WriteStorage<'a, Position>,
    player: ReadStorage<'a, Player>,
    action: ReadStorage<'a, Action>,
    name: ReadStorage<'a, Name>,
    item: ReadStorage<'a, Item>,
    energy: WriteStorage<'a, Energy>,
    living: WriteStorage<'a, Living>,
    ai: WriteStorage<'a, Ai>,

    messages: Write<'a, Messages>,
    targeting: WriteExpect<'a, Option<Targeting>>,
    fov_map: Option<ReadExpect<'a, Arc<Mutex<FovMap>>>>,

    entities: Entities<'a>,
}

impl<'a> System<'a> for UseItemSystem {
    type SystemData = UseItemSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (actor, inventory, action) in (&data.entities, &mut data.inventory, &data.action).join()
        {
            match *action {
                Action::UseFromInventory { inventory_index } => {
                    if inventory_index >= inventory.0.len() {
                        continue;
                    }
                    let item_entity = *inventory.0.get(inventory_index).unwrap();
                    let opt_item = data.item.get(item_entity);
                    if let Some(item) = opt_item {
                        let result = match item {
                            Item::Heal => cast_heal(
                                action,
                                &mut data.living,
                                &mut data.energy,
                                &data.player,
                                &mut data.messages,
                            ),
                            Item::Lightning => UseResult::Cancelled, //cast_lightning,
                            Item::Confuse => UseResult::Cancelled,   //target_confuse,
                            Item::Fireball => UseResult::Cancelled,  //target_fireball,
                        };

                        match result {
                            UseResult::UsedUp => {
                                // destroy after use, unless it was cancelled for some reason
                                inventory.0.retain(|&x| x != item_entity);
                                data.entities.delete(item_entity).unwrap();
                            }
                            UseResult::Cancelled => {
                                data.messages.push("Cancelled", colors::WHITE);
                            }
                            UseResult::Targeting => (),
                            UseResult::NotEnoughEnergy => (),
                        }
                    } else {
                        data.messages.push(
                            format!(
                                "The {} cannot be used.",
                                data.name.get(inventory.0[inventory_index]).unwrap().0
                            ),
                            colors::WHITE,
                        );
                    }
                }
                _ => (),
            }
        }
    }
}

enum UseResult {
    UsedUp,
    Cancelled,
    NotEnoughEnergy,
    Targeting,
}

const HEAL_AMOUNT: i32 = 4;

fn cast_heal(
    action: &Action,
    living: &mut WriteStorage<Living>,
    energy: &mut WriteStorage<Energy>,
    player: &ReadStorage<Player>,
    messages: &mut Write<Messages>,
) -> UseResult {
    // heal the player
    if let Some((living, energy, _)) = (living, energy, player).join().next() {
        if living.hp == living.max_hp {
            messages.push("You are already at full health.", colors::RED);
            UseResult::Cancelled
        } else if energy.consume(action.energy_cost()) {
            messages.push("Your wounds start to feel better!", colors::LIGHT_VIOLET);
            heal(living, HEAL_AMOUNT);
            UseResult::UsedUp
        } else {
            UseResult::NotEnoughEnergy
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

fn cast_lightning(_entity: Entity, data: &mut UseItemSystemData, action: Action) -> UseResult {
    // find closest enemy (inside a maximum range and damage it)
    let monster = closest_monster(LIGHTNING_RANGE, data);
    if let Some(monster) = monster {
        let energy = (&mut data.energy, &data.player).join().next().unwrap().0;
        if energy.consume(action.energy_cost()) {
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
            UseResult::NotEnoughEnergy
        }
    } else {
        // no enemy found within maximum range
        data.messages
            .push("No enemy is close enough to strike.", colors::RED);
        UseResult::Cancelled
    }
}

const CONFUSE_RANGE: f32 = 8.0;
const CONFUSE_NUM_TURNS: i32 = 10;

fn target_confuse(entity: Entity, data: &mut UseItemSystemData, action: Action) -> UseResult {
    *data.targeting = Some(Targeting {
        used_item: entity,
        kind: TargetingKind::Monster,
        max_range: Some(CONFUSE_RANGE),
    });
    data.messages.push(
        "Left-click an enemy to confuse it, or right-click to cancel.",
        colors::CYAN,
    );
    UseResult::Targeting
}

fn cast_confuse(position: &Position, data: &mut UseItemSystemData) -> UseResult {
    let monster = (&data.position, &data.ai, &data.living, &data.entities)
        .join()
        .find(|j| j.0 == position && j.2.alive)
        .unwrap()
        .3;
    let old_ai = data.ai.get(monster).map(Clone::clone).unwrap_or(Ai::Basic);
    data.ai
        .insert(
            monster,
            Ai::Confused {
                previous_ai: Box::new(old_ai),
                num_turns: CONFUSE_NUM_TURNS,
            },
        )
        .unwrap();
    data.messages.push(
        format!(
            "The eyes of {} look vacant, as he starts to stumble around!",
            &data.name.get(monster).unwrap().0
        ),
        colors::LIGHT_GREEN,
    );
    UseResult::UsedUp
}

const FIREBALL_RADIUS: f32 = 3.0;
const FIREBALL_DAMAGE: i32 = 12;

fn target_fireball(entity: Entity, data: &mut UseItemSystemData, action: Action) -> UseResult {
    *data.targeting = Some(Targeting {
        used_item: entity,
        kind: TargetingKind::Tile,
        max_range: None,
    });
    data.messages.push(
        "Left-click a target tile for the fireball, or right-click to cancel.",
        colors::CYAN,
    );
    UseResult::Targeting
}

fn cast_fireball(position: &Position, data: &mut UseItemSystemData) -> UseResult {
    data.messages.push(
        format!(
            "The fireball explodes, burning everything within {} tiles!",
            FIREBALL_RADIUS
        ),
        colors::ORANGE,
    );
    for (target_position, living, name) in (&data.position, &mut data.living, &data.name).join() {
        if position.distance_to(&target_position) <= FIREBALL_RADIUS && living.alive {
            data.messages.push(
                format!(
                    "The {} gets burned for {} hit points.",
                    name.0, FIREBALL_DAMAGE
                ),
                colors::ORANGE,
            );
            living.hp -= FIREBALL_DAMAGE;
        }
    }
    UseResult::UsedUp
}

/// find closest enemy, up to a maximum range, and in the player's FOV
fn closest_monster(max_range: i32, data: &mut UseItemSystemData) -> Option<Entity> {
    let mut closest_enemy = None;
    let mut closest_dist = (max_range + 1) as f32; // start with (slightly more than) maximum range

    let player_pos = (&data.position, &data.player).join().next().unwrap().0;

    let fov_map_mutex = data.fov_map.as_ref().unwrap().clone();
    let fov_map = &*fov_map_mutex.lock().unwrap();

    for (entity, pos, _, _) in (&data.entities, &data.position, &data.living, &data.ai)
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
