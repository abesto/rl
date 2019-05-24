use specs::*;
use systems::*;
use tcod::colors;
use tcod::input::Mouse;
use tcod::input::{self, Event, Key};

mod components;
mod mapgen;
mod resources;
mod systems;

use crate::components::*;
use crate::resources::map::Map;
use crate::resources::messages::Messages;
use crate::resources::ui::{self, UIConfig, PANEL_HEIGHT};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

impl Default for PlayerAction {
    fn default() -> PlayerAction {
        PlayerAction::DidntTakeTurn
    }
}

fn main() {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(LocationHistorySystem, "location_history", &[])
        .with_barrier() // Player turn
        .with(AttackSystem, "player_attack", &["input"])
        .with(CollisionSystem, "player_collision", &["player_attack"])
        .with(
            MovementSystem,
            "player_movement",
            &["player_collision", "location_history"],
        )
        .with(FovSystem, "fov", &["player_movement"])
        .with(MonsterDeathSystem, "monster_death", &["player_attack"])
        .with_barrier() // Monster turn
        .with(AISystem, "ai", &[])
        .with(AttackSystem, "monster_attack", &["ai"])
        .with(CollisionSystem, "monster_collision", &["monster_attack"])
        .with(MovementSystem, "monster_movement", &["monster_collision"])
        .with(PlayerDeathSystem, "player_death", &["monster_attack"])
        .with_barrier() // Rendering
        .with(FogOfWarSystem, "fog_of_war", &["fov"])
        .with_thread_local(RenderSystem)
        .build();

    // Wire it all up
    world.add_resource(PlayerAction::default());
    world.register::<Item>();
    dispatcher.setup(&mut world.res);

    // Initialize UI state
    let ui_config = UIConfig::new();
    let ui_state = ui::init(ui_config);
    world.add_resource(ui_state);
    world.add_resource(Mouse::default());

    // Initialize the message log
    let mut messages = Messages::new(PANEL_HEIGHT as usize);
    messages.push(
        "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
        colors::RED,
    );
    world.add_resource(messages);

    // Set up the map
    Map::new_random(&mut world);
    let map = || world.read_resource::<Map>();
    let fov_map = systems::fov::new_fov_map(&map().tiles);
    let spawn_point = map().spawn_point.clone();
    world.add_resource(fov_map);

    // Create player ;)
    world
        .create_entity()
        .with(spawn_point)
        .with(Velocity::new())
        .with(Visual {
            char: '@',
            color: colors::WHITE,
        })
        .with(Collider)
        .with(Player)
        .with(Name::new("player"))
        .with(PreviousPosition { x: -1, y: -1 })
        .with(Living {
            alive: true,
            max_hp: 30,
            hp: 30,
            defense: 2,
        })
        .with(Power(5))
        .with(Inventory::new())
        .build();

    // And start the game
    dispatcher.dispatch(&world.res);
    while *world.read_resource::<PlayerAction>() != PlayerAction::Exit {
        world.maintain();
        {
            match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
                Some((_, Event::Mouse(m))) => {
                    *world.write_resource() = m;
                    *world.write_resource::<Option<Key>>() = None;
                }
                Some((_, Event::Key(k))) => *world.write_resource() = Some(k),
                _ => *world.write_resource::<Option<Key>>() = None,
            }
        }
        dispatcher.dispatch(&world.res);
    }
}
