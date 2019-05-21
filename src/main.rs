use specs::*;
use systems::*;
use tcod::colors;
use tcod::input::Key;

mod components;
mod map;
mod mapgen;
mod systems;
mod ui;

use crate::map::Map;
use components::*;
use ui::UIState;

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
        .with(AttackSystem, "attack", &["input"])
        .with(CollisionSystem, "collision", &["attack"])
        .with(
            MovementSystem,
            "movement",
            &["collision", "location_history"],
        )
        .with(AISystem, "ai", &["movement"])
        .with(FovSystem, "fov", &["movement"])
        .with(FogOfWarSystem, "fog_of_war", &["fov"])
        .with_thread_local(RenderSystem)
        .build();

    // Wire it all up
    world.add_resource(PlayerAction::default());
    world.register::<Alive>();
    world.register::<Name>();
    dispatcher.setup(&mut world.res);

    // Initialize UI state
    let ui_config = ui::UIConfig::new();
    world.add_resource(ui::init(ui_config));

    // Set up the map
    map::Map::new_random(&mut world);
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
        .with(Player)
        .with(Name::new("player"))
        .with(PreviousPosition { x: -1, y: -1 })
        .with(Alive(true))
        .build();

    // And start the game
    dispatcher.dispatch(&world.res);
    while *world.read_resource::<PlayerAction>() != PlayerAction::Exit {
        world.maintain();
        *world.write_resource::<Option<Key>>() = Some(
            world
                .write_resource::<UIState>()
                .consoles
                .root
                .wait_for_keypress(true),
        );
        dispatcher.dispatch(&world.res);
    }
}
