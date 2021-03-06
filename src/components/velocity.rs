use std::convert::From;

use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;

// Intentionally not implemented as a vector; that makes things more complex, it can come later

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Heading {
    North,
    East,
    South,
    West,
}

impl Default for Heading {
    fn default() -> Heading {
        Heading::North
    }
}

#[derive(Component, Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Velocity {
    pub heading: Heading,
    pub magnitude: u8,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity {
            heading: Heading::North,
            magnitude: 0,
        }
    }

    pub fn unit(heading: Heading) -> Velocity {
        Velocity {
            heading,
            magnitude: 1,
        }
    }

    pub fn with_heading(&self, heading: Heading) -> Velocity {
        Velocity {
            heading,
            magnitude: self.magnitude,
        }
    }

    pub fn with_magnitude(&self, magnitude: u8) -> Velocity {
        Velocity {
            heading: self.heading.clone(),
            magnitude,
        }
    }
}

impl From<Heading> for Velocity {
    fn from(heading: Heading) -> Velocity {
        Velocity {
            heading,
            magnitude: 1,
        }
    }
}

impl From<(i32, i32)> for Velocity {
    fn from((x, y): (i32, i32)) -> Velocity {
        if x != 0 && y != 0 {
            panic!("Velocity::from::<(i32,i32)> supports only diagonal movement");
        }
        if x < 0 {
            return Velocity {
                heading: Heading::West,
                magnitude: -x as u8,
            };
        } else if x > 0 {
            return Velocity {
                heading: Heading::East,
                magnitude: x as u8,
            };
        } else if y < 0 {
            return Velocity {
                heading: Heading::North,
                magnitude: -y as u8,
            };
        } else if y > 0 {
            return Velocity {
                heading: Heading::South,
                magnitude: y as u8,
            };
        }
        Velocity {
            heading: Heading::North,
            magnitude: 0,
        }
    }
}
