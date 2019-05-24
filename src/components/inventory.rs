use specs::{Component, Entity, HashMapStorage};
use specs_derive::Component;

#[derive(Default, Component)]
#[storage(HashMapStorage)]
pub struct Inventory(pub Vec<Entity>);

impl Inventory {
    pub fn new() -> Inventory {
        Inventory(Vec::new())
    }
}
