use shred_derive::SystemData;
use specs::prelude::*;
use tcod::input::Key;

use crate::components::{Player, Velocity};
use crate::ui::UIState;

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    key: Read<'a, Option<Key>>,
    velocity: WriteStorage<'a, Velocity>,
    player: ReadStorage<'a, Player>,

    ui: WriteExpect<'a, UIState>,
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use crate::components::velocity::Heading::*;
        use specs::Join;
        use tcod::input::KeyCode::*;

        for (vel, _) in (&mut data.velocity, &data.player).join() {
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
                    Key { code: Up, .. } => *vel = Velocity::unit(North),
                    Key { code: Right, .. } => *vel = Velocity::unit(East),
                    Key { code: Down, .. } => *vel = Velocity::unit(South),
                    Key { code: Left, .. } => *vel = Velocity::unit(West),
                    _ => (),
                }
            }
        }
    }
}
