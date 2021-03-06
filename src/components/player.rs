use serde::{Deserialize, Serialize};
use specs::{Component, HashMapStorage};
use specs_derive::Component;

#[derive(PartialEq, Component, Debug, Clone, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub struct Player;

impl Player {
    pub fn new() -> Player {
        Player
    }
}
