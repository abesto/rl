use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Name(pub String);

impl Name {
    pub fn new(s: &str) -> Name {
        Name(s.into())
    }
}
