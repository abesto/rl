use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Default, Component, Serialize, Deserialize, Clone)]
pub struct Collider;

impl Collider {
    pub fn new() -> Collider {
        Collider
    }
}
