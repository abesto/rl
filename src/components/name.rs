use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(VecStorage)]
pub struct Name(pub String);

impl Name {
    pub fn new(s: &str) -> Name {
        Name(s.into())
    }
}
