use serde::{Deserialize, Serialize};
use specs::{prelude::*, Component};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Power(pub i32);
