use std::convert::From;

use specs::{Component, VecStorage};
use specs_derive::Component;

// Intentionally not implemented as a vector; that makes things more complex, it can come later

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Component, Debug, PartialEq, Clone, Default)]
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

impl From<(i32, i32)> for Velocity {
    fn from((x, y): (i32, i32)) -> Velocity {
        if x != 0 && y != 0 {
            panic!("Velocity::from::<(i32,i32)> supports only diagonal movement");
        }
        if x < 0 {
            return Velocity {
                heading: Heading::West,
                magnitude: -x as usize,
            };
        } else if x > 0 {
            return Velocity {
                heading: Heading::East,
                magnitude: x as usize,
            };
        } else if y < 0 {
            return Velocity {
                heading: Heading::North,
                magnitude: -y as usize,
            };
        } else if y > 0 {
            return Velocity {
                heading: Heading::South,
                magnitude: y as usize,
            };
        }
        Velocity {
            heading: Heading::North,
            magnitude: 0,
        }
    }
}
