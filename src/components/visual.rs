use specs::{Component, VecStorage};
use specs_derive::Component;
use tcod::colors::Color;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Visual {
    pub char: char,
    pub color: Color,
}
