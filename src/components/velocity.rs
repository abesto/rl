use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}
