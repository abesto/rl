use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Alive(pub bool);
