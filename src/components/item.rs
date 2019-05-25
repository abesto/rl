use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Item {
    Heal,
    Lightning,
}
