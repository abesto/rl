use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;
use tcod::colors::Color;

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Visual {
    pub char: char,
    pub color: Color,
    pub always_visible: bool,
}
