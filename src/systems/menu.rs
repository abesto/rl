use shred_derive::SystemData;
use specs::prelude::*;

use crate::{
    components::*,
    resources::{
        input_action::InputAction,
        menu::{Menu, MenuKind},
        state::State,
    },
};

pub struct MenuSystem;

#[derive(SystemData)]
pub struct MenuSystemData<'a> {
    input_action: ReadExpect<'a, InputAction>,

    menu: WriteExpect<'a, Option<Menu>>,
    state: Write<'a, State>,
}

impl<'a> System<'a> for MenuSystem {
    type SystemData = MenuSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        match *data.input_action {
            InputAction::MainMenu => {
                *data.menu = Some(Menu {
                    items: vec![
                        "Play a new game".to_string(),
                        "Continue last game".to_string(),
                        "Quit".to_string(),
                    ],
                    header: "".to_string(),
                    width: 24,
                    kind: MenuKind::Main,
                });
                *data.state = State::MainMenu;
            }
            _ => (),
        }
    }
}
