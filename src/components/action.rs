use serde::{Deserialize, Serialize};
use specs::{prelude::*, Component};
use specs_derive::Component;

use crate::components::Velocity;

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Skip {
        ticks: u8,
    },
    WaitForInput,
    MoveOrMelee {
        velocity: Velocity,
        attack_player: bool,
        attack_monsters: bool,
    },
    PickUp,
    Drop {
        inventory_index: usize,
    },
    UseFromInventory {
        inventory_index: usize,
    },
}

impl Action {
    pub fn energy_cost(self: &Self) -> u8 {
        use crate::components::Action::*;
        match self {
            Action::Skip { ticks } => *ticks,
            MoveOrMelee { velocity, .. } => velocity.magnitude,
            PickUp => 1,
            WaitForInput => 0,
            Drop { .. } => 1,
            UseFromInventory { .. } => 1,
        }
    }

    pub fn noop() -> Action {
        Action::Skip { ticks: 0 }
    }
}
