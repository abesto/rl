use specs::{Component, HashMapStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Player;
