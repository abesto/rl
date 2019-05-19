use crate::ui::UIState;
use shred_derive::SystemData;
use specs::prelude::*;
use tcod::input::Key;

use crate::components::Velocity;

pub struct InputSystem;

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    key: Read<'a, Option<Key>>,
    velocity: WriteStorage<'a, Velocity>,

    ui: WriteExpect<'a, UIState>,
}

impl<'a> System<'a> for InputSystem {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;
        use tcod::input::KeyCode::*;

        for vel in (&mut data.velocity).join() {
            if let Some(k) = *data.key {
                match k {
                    Key {
                        code: Enter,
                        alt: true,
                        ..
                    } => {
                        // Alt+Enter: toggle fullscreen
                        data.ui.config.fullscreen = !data.ui.config.fullscreen;
                    }
                    Key { code: Escape, .. } => data.ui.exit_requested = true,
                    Key { code: Up, .. } => vel.y -= 1,
                    Key { code: Down, .. } => vel.y += 1,
                    Key { code: Left, .. } => vel.x -= 1,
                    Key { code: Right, .. } => vel.x += 1,
                    _ => (),
                }
            }
        }
    }
}
