use std::convert::From;

use specs::{Component, VecStorage};
use specs_derive::Component;

// Intentionally not implemented as a vector; that makes things more complex, it can come later

#[derive(Debug, Clone)]
pub enum Heading {
    North,
    East,
    South,
    West,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub heading: Heading,
    pub magnitude: usize,
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

    pub fn with_magnitude(&self, magnitude: usize) -> Velocity {
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
