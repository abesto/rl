use specs::*;
use systems::*;
use tcod::colors;
use tcod::input::Key;

mod components;
mod map;
mod mapgen;
mod systems;
mod ui;

use components::*;
use ui::UIState;

fn main() {
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(MovementSystem, "movement", &["input"])
        .with_thread_local(RenderSystem)
        .build();

    let mut world = World::new();

    // Initialize UI state
    let ui_config = ui::UIConfig::new();
    let width = ui_config.width;
    let height = ui_config.height;
    world.add_resource(ui::init(ui_config));

    // Set up the map
    let map = map::Map::new_random();
    let spawn_point = map.spawn_point.clone();
    world.add_resource(map);

    // Wire it all up
    dispatcher.setup(&mut world.res);

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
        .build();

    // And an NPC
    world
        .create_entity()
        .with(Position {
            x: width / 2 - 5,
            y: height / 2,
        })
        .with(Velocity::new())
        .with(Visual {
            char: '@',
            color: colors::YELLOW,
        })
        .build();

    // And start the game
    dispatcher.dispatch(&world.res);
    while !world.read_resource::<UIState>().exit_requested {
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
