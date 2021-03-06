use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Living {
    pub alive: bool,
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
}
