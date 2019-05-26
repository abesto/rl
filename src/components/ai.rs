use serde::{Deserialize, Serialize};
use specs::{prelude::*, Component};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ai {
    Basic,
    Confused {
        previous_ai: Box<Ai>,
        num_turns: i32,
    },
}
