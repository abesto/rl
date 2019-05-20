use specs::{Component, NullStorage};
use specs_derive::Component;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Collider;
