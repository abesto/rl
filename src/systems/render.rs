use crate::components::{Position, Visual};
use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;

pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    position: ReadStorage<'a, Position>,
    visual: ReadStorage<'a, Visual>,
    ui: WriteExpect<'a, crate::ui::UIState>,
}

impl RenderSystem {
    fn draw_object(offscreen: &mut Offscreen, position: &Position, visual: &Visual) {
        offscreen.set_default_foreground(visual.color);
        offscreen.put_char(position.x, position.y, visual.char, BackgroundFlag::None);
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        let ui = &mut *data.ui;
        let root = &mut ui.consoles.root;

        let offscreen_mutex = ui.consoles.offscreen.clone();
        let offscreen = &mut *offscreen_mutex.lock().unwrap();

        ui.config.apply(root);

        offscreen.set_default_foreground(WHITE);
        offscreen.clear();

        for (position, visual) in (&data.position, &data.visual).join() {
            Self::draw_object(offscreen, position, visual);
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
