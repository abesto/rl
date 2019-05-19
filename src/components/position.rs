use std::ops::Add;

use crate::components::Velocity;
use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<&Velocity> for Position {
    fn from(velocity: &Velocity) -> Position {
        use crate::components::velocity::Heading::*;
        let n = velocity.magnitude as i32;
        match velocity.heading {
            North => Position { x: 0, y: -n },
            East => Position { x: n, y: 0 },
            South => Position { x: 0, y: n },
            West => Position { x: -n, y: 0 },
        }
    }
}

impl Add<&Velocity> for &Position {
    type Output = Position;

    fn add(self, vel: &Velocity) -> Position {
        let other: Position = vel.into();
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(VecStorage)]
pub struct PreviousPosition {
    pub x: i32,
    pub y: i32,
}
