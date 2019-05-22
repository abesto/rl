use shred_derive::SystemData;
use specs::prelude::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

use crate::components::velocity::Heading::*;
use crate::components::{Alive, Player, Velocity};
use crate::ui::UIState;
use crate::PlayerAction;
use crate::PlayerAction::*;

#[derive(SystemData)]
pub struct InputSystemData<'a> {
    key: Read<'a, Option<Key>>,

    alive: ReadStorage<'a, Alive>,
    player: ReadStorage<'a, Player>,
    velocity: WriteStorage<'a, Velocity>,

    ui: WriteExpect<'a, UIState>,
    action: WriteExpect<'a, PlayerAction>,
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = InputSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (vel, alive, _) in (&mut data.velocity, &data.alive, &data.player).join() {
            if let Some(k) = *data.key {
                *data.action = match (k, alive.0) {
                    (
                        Key {
                            code: Enter,
                            alt: true,
                            ..
                        },
                        _,
                    ) => {
                        // Alt+Enter: toggle fullscreen
                        data.ui.config.fullscreen = !data.ui.config.fullscreen;
                        DidntTakeTurn
                    }
                    (Key { code: Escape, .. }, _) => Exit,
                    (Key { code: Up, .. }, true) => {
                        *vel = Velocity::unit(North);
                        TookTurn
                    }
                    (Key { code: Right, .. }, true) => {
                        *vel = Velocity::unit(East);
                        TookTurn
                    }
                    (Key { code: Down, .. }, true) => {
                        *vel = Velocity::unit(South);
                        TookTurn
                    }
                    (Key { code: Left, .. }, true) => {
                        *vel = Velocity::unit(West);
                        TookTurn
                    }
                    _ => DidntTakeTurn,
                }
            }
        }
    }
}
