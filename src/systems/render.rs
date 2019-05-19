use crate::components::Position;
use shred_derive::SystemData;
use specs::prelude::*;

pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    position: ReadStorage<'a, Position>,
    ui: WriteExpect<'a, crate::ui::UIState>,
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;
        use tcod::colors::*;
        use tcod::console::*;

        let ui = &mut *data.ui;
        let root = &mut ui.consoles.root;

        let offscreen_mutex = ui.consoles.offscreen.clone();
        let mut offscreen = offscreen_mutex.lock().unwrap();

        ui.config.apply(root);

        offscreen.set_default_foreground(WHITE);
        offscreen.clear();

        for pos in (&data.position).join() {
            offscreen.put_char(pos.x, pos.y, '@', BackgroundFlag::None);
        }

        blit(
            &*offscreen,
            (0, 0),
            (ui.config.width, ui.config.height),
            root,
            (0, 0),
            1.0,
            1.0,
        );

        root.flush();
    }
}
