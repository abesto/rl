use specs::prelude::*;
use specs::Component;
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Ai {
    Basic,
    Confused {
        previous_ai: Box<Ai>,
        num_turns: i32,
    },
}
