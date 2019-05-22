use specs::prelude::*;
use specs::Component;
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Power(pub i32);
