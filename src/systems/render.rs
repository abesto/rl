use crate::components::{Position, Visual};
use crate::map::Map;
use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    position: ReadStorage<'a, Position>,
    visual: ReadStorage<'a, Visual>,

    map: ReadExpect<'a, Map>,
    ui: WriteExpect<'a, crate::ui::UIState>,
}

impl RenderSystem {
    fn draw_map(offscreen: &mut Offscreen, map: &Map) {
        for x in 0..map.tiles.len() {
            for y in 0..map.tiles[x].len() {
                let wall = map.tiles[x][y].block_sight;
                offscreen.set_char_background(
                    x as i32,
                    y as i32,
                    if wall {
                        COLOR_DARK_WALL
                    } else {
                        COLOR_DARK_GROUND
                    },
                    BackgroundFlag::Set,
                );
            }
        }
    }

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

        let map_mutex = ui.consoles.map.clone();
        let map = &mut *map_mutex.lock().unwrap();

        ui.config.apply(root);

        map.set_default_foreground(WHITE);
        map.clear();

        Self::draw_map(map, &data.map);
        for (position, visual) in (&data.position, &data.visual).join() {
            Self::draw_object(map, position, visual);
        }

        blit(
            &*map,
            (0, 0),
            (crate::map::MAP_WIDTH, crate::map::MAP_HEIGHT),
            root,
            (0, 0),
            1.0,
            1.0,
        );

        root.flush();
    }
}
