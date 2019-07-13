use shred_derive::SystemData;
use specs::prelude::*;

use crate::components::*;

pub struct SkipSystem;

#[derive(SystemData)]
pub struct SkipSystemData<'a> {
    action: ReadStorage<'a, Action>,
    energy: WriteStorage<'a, Energy>,
}

impl<'a> System<'a> for SkipSystem {
    type SystemData = SkipSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (action, energy) in (&data.action, &mut data.energy).join() {
            match action {
                Action::Skip { ticks } => {
                    energy.consume(*ticks);
                }
                _ => (),
            }
        }
    }
}
