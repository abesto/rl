use serde::{Deserialize, Serialize};
use specs::{prelude::*, Component};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Energy(i8);

impl Energy {
    pub fn new() -> Energy {
        Energy(0)
    }

    pub fn gain(self: &mut Self, amount: i8) {
        self.0 += amount;
    }

    pub fn can_act(self: &Self) -> bool {
        self.0 > 0
    }

    pub fn consume(self: &mut Self, amount: u8) -> bool {
        if self.can_act() {
            self.0 -= amount as i8;
            true
        } else {
            false
        }
    }
}
