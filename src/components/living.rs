use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Living {
    pub alive: bool,
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
}

impl Living {
    pub fn take_damage(&mut self, damage: i32) {
        if damage > 0 {
            self.hp -= damage;
        }
    }
}
