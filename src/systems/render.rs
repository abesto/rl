use crate::components::Position;
use specs::{ReadStorage, System, WriteExpect};

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteExpect<'a, tcod::console::Root>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut root, pos_storage): Self::SystemData) {
        use specs::Join;
        use tcod::colors::*;
        use tcod::console::*;

        root.set_default_foreground(WHITE);
        root.clear();

        for pos in (&pos_storage).join() {
            root.put_char(pos.x, pos.y, '@', BackgroundFlag::None);
        }

        root.flush();
    }
}
