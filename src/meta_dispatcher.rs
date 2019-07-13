use specs::prelude::*;

type Predicate = fn(&World) -> bool;

pub enum RunWhile {
    Once,
    Predicate(Predicate),
}

pub struct Entry<'a, 'b> {
    run_while: RunWhile,
    dispatcher: Dispatcher<'a, 'b>,
}

pub struct MetaDispatcher<'a, 'b> {
    entries: Vec<Entry<'a, 'b>>,
}

impl<'a, 'b> MetaDispatcher<'a, 'b> {
    pub fn new() -> Self {
        MetaDispatcher { entries: vec![] }
    }

    pub fn add(self: &mut Self, run_while: RunWhile, dispatcher: Dispatcher<'a, 'b>) {
        self.entries.push(Entry {
            run_while,
            dispatcher,
        })
    }

    pub fn once(self: &mut Self, dispatcher: Dispatcher<'a, 'b>) {
        self.add(RunWhile::Once, dispatcher);
    }

    pub fn run_while(self: &mut Self, predicate: Predicate, dispatcher: Dispatcher<'a, 'b>) {
        self.add(RunWhile::Predicate(predicate), dispatcher);
    }

    pub fn dispatch(self: &mut Self, world: &World) {
        for entry in &mut self.entries {
            match entry.run_while {
                RunWhile::Once => entry.dispatcher.dispatch(&world.res),
                RunWhile::Predicate(pred) => {
                    while pred(world) {
                        entry.dispatcher.dispatch(&world.res);
                    }
                }
            }
        }
    }

    pub fn setup(self: &mut Self, res: &mut Resources) {
        for entry in &mut self.entries {
            entry.dispatcher.setup(res);
        }
    }
}
