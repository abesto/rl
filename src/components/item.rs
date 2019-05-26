use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Heal,
    Lightning,
    Confuse,
    Fireball,
}
