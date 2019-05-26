use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Default, Component, Serialize, Deserialize, Clone)]
pub struct Collider(bool);

// Embed a bool so that Serde serializes to non-null :(

impl Collider {
    pub fn new() -> Collider {
        Collider(true)
    }
}
