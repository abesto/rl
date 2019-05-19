mod components;
mod systems;
mod ui;

use crate::ui::UIState;
use components::*;
use specs::*;
use systems::*;
use tcod::input::Key;

fn main() {
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(MovementSystem, "movement", &["input"])
        .with_thread_local(RenderSystem)
        .build();

    let mut world = World::new();

    let ui_config = ui::UIConfig::new();
    world.add_resource(ui::init(ui_config));

    dispatcher.setup(&mut world.res);

    // Create player ;)
    world
        .create_entity()
        .with(Position { x: 25, y: 25 })
        .with(Velocity { x: 0, y: 0 })
        .build();

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
