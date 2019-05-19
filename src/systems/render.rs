use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors;
use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;

use crate::components::{Position, PreviousPosition, Visual};
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};
const COLOR_LIGHT_WALL: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
const COLOR_LIGHT_GROUND: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    entity: Entities<'a>,
    prev_position: ReadStorage<'a, PreviousPosition>,
    position: ReadStorage<'a, Position>,
    visual: ReadStorage<'a, Visual>,

    map: ReadExpect<'a, Map>,
    fov_map: ReadExpect<'a, Arc<Mutex<FovMap>>>,
    ui: WriteExpect<'a, crate::ui::UIState>,
}

impl RenderSystem {
    fn draw_object(offscreen: &mut Offscreen, position: &Position, visual: &Visual) {
        offscreen.set_default_foreground(visual.color);
        offscreen.put_char(position.x, position.y, visual.char, BackgroundFlag::None);
    }

    fn draw_movement_shadow(
        offscreen: &mut Offscreen,
        prev_pos: &PreviousPosition,
        visual: &Visual,
    ) {
        if prev_pos.x >= 0 && prev_pos.y >= 0 {
            Self::draw_object(
                offscreen,
                &Position {
                    x: prev_pos.x,
                    y: prev_pos.y,
                },
                &Visual {
                    char: visual.char,
                    color: colors::DARK_GREY,
                },
            );
        }
    }

    fn draw_fov(offscreen: &mut Offscreen, map: &Map, fov_map: &FovMap) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if !map.tiles[x as usize][y as usize].explored {
                    continue;
                }
                let visible = fov_map.is_in_fov(x, y);
                let wall = map.tiles[x as usize][y as usize].block_sight;
                let color = match (visible, wall) {
                    // outside of field of view:
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,
                    // inside fov:
                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
                };
                offscreen.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
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

        let fov_map_mutex = data.fov_map.clone();
        let fov_map = &*fov_map_mutex.lock().unwrap();

        ui.config.apply(root);

        map.set_default_foreground(WHITE);
        map.clear();

        Self::draw_fov(map, &data.map, fov_map);
        for (entity, position, visual) in (&data.entity, &data.position, &data.visual).join() {
            if !fov_map.is_in_fov(position.x, position.y) {
                continue;
            }
            // Draw movement shadow for debugging
            if let Some(prev_pos) = data.prev_position.get(entity) {
                Self::draw_movement_shadow(map, prev_pos, visual);
            }
            // Draw the object proper
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
