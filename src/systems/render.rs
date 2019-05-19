use crate::components::Position;
use shred_derive::SystemData;
use specs::prelude::*;

pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    position: ReadStorage<'a, Position>,

    ui_config: ReadExpect<'a, crate::ui::UiConfig>,
    root: WriteExpect<'a, tcod::console::Root>,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;
        use tcod::colors::*;
        use tcod::console::*;

        data.ui_config.apply(&mut *data.root);

        data.root.set_default_foreground(WHITE);
        data.root.clear();

        for pos in (&data.position).join() {
            data.root.put_char(pos.x, pos.y, '@', BackgroundFlag::None);
        }

        data.root.flush();
    }
}
