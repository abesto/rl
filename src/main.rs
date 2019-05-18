extern crate specs;
#[macro_use]
extern crate specs_derive;

mod ui;

use specs::DispatcherBuilder;
use specs::{Builder, World};
use specs::{Component, VecStorage};
use specs::{ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use tcod::input::Key;

use tcod::console::Root;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: i32,
    y: i32,
}

struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadExpect<'a, Option<Key>>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (key, mut vel_storage): Self::SystemData) {
        use specs::Join;
        use tcod::input::KeyCode::*;

        for vel in (&mut vel_storage).join() {
            key.map(|k| match k {
                Key { code: Up, .. } => vel.y -= 1,
                Key { code: Down, .. } => vel.y += 1,
                Key { code: Left, .. } => vel.x -= 1,
                Key { code: Right, .. } => vel.x += 1,
                _ => (),
            });
        }
    }
}

struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (mut pos_storage, mut vel_storage): Self::SystemData) {
        use specs::Join;

        for (mut pos, mut vel) in (&mut pos_storage, &mut vel_storage).join() {
            pos.x += vel.x;
            pos.y += vel.y;
            vel.x = 0;
            vel.y = 0;
        }
    }
}

struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteExpect<'a, tcod::console::Root>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut root, pos_storage): Self::SystemData) {
        use specs::Join;
        use tcod::colors::*;
        use tcod::console::*;

        root.set_default_foreground(WHITE);
        root.clear();

        for pos in (&pos_storage).join() {
            root.put_char(pos.x, pos.y, '@', BackgroundFlag::None);
        }

        root.flush();
    }
}

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
