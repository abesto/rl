use std::ops::Add;

use rand::Rng;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;

use crate::components::Velocity;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Position {
    pub fn move_towards(&self, target: &Position) -> Velocity {
        // vector from this object to the target, and distance
        let dx = target.x - self.x;
        let dy = target.y - self.y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        // normalize it to length 1 (preserving direction), then round it and
        // convert to integer so the movement is restricted to the map grid
        let mut dx = (dx as f32 / distance).round() as i32;
        let mut dy = (dy as f32 / distance).round() as i32;

        // the player cannot move diagonally, so it's only fair the monsters can't either
        if dx != 0 && dy != 0 {
            if rand::thread_rng().gen_bool(0.5) {
                dy = 0;
            } else {
                dx = 0;
            }
        }

        Velocity::from((dx, dy))
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct PreviousPosition {
    pub x: i32,
    pub y: i32,
}
