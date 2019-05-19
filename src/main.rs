mod components;
mod systems;
mod ui;

use crate::ui::UIState;
use components::*;
use specs::*;
use systems::*;
use tcod::colors;
use tcod::input::Key;

fn main() {
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(MovementSystem, "movement", &["input"])
        .with_thread_local(RenderSystem)
        .build();

    let mut world = World::new();

    let ui_config = ui::UIConfig::new();
    let width = ui_config.width;
    let height = ui_config.height;
    world.add_resource(ui::init(ui_config));
    dispatcher.setup(&mut world.res);

    // Create player ;)
    world
        .create_entity()
        .with(Position {
            x: width / 2,
            y: height / 2,
        })
        .with(Velocity { x: 0, y: 0 })
        .with(Visual {
            char: '@',
            color: colors::WHITE,
        })
        .build();

    // And an NPC
    world
        .create_entity()
        .with(Position {
            x: width / 2 - 5,
            y: height / 2,
        })
        .with(Velocity { x: 0, y: 0 })
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
