extern crate specs;
extern crate tcod;
#[macro_use]
extern crate specs_derive;

mod components;
mod systems;
mod ui;

use components::*;
use specs::*;
use systems::*;
use tcod::console::Root;
use tcod::input::Key;

fn main() {
    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem, "input", &[])
        .with(MovementSystem, "movement", &["input"])
        .with_thread_local(RenderSystem)
        .build();

    let mut world = World::new();

    world.add_resource(ui::init());
    world.add_resource::<Option<Key>>(None);

    dispatcher.setup(&mut world.res);

    // Create player ;)
    world
        .create_entity()
        .with(Position { x: 25, y: 25 })
        .with(Velocity { x: 0, y: 0 })
        .build();

    while !world.read_resource::<Root>().window_closed() {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
        *world.write_resource::<Option<Key>>() =
            Some(world.write_resource::<Root>().wait_for_keypress(true));
    }
}
