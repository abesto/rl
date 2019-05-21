use specs::prelude::*;
use specs::Component;
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Fighter {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}
