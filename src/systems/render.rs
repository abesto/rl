use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;

use crate::components::*;
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
    living: ReadStorage<'a, Living>,
    player: ReadStorage<'a, Player>,
    position: ReadStorage<'a, Position>,
    visual: ReadStorage<'a, Visual>,
    collider: ReadStorage<'a, Collider>,
    entities: Entities<'a>,

    map: ReadExpect<'a, Map>,
    fov_map: ReadExpect<'a, Arc<Mutex<FovMap>>>,
    ui: WriteExpect<'a, crate::ui::UIState>,
}

impl RenderSystem {
    fn draw_object(offscreen: &mut Offscreen, position: &Position, visual: &Visual) {
        offscreen.set_default_foreground(visual.color);
        offscreen.put_char(position.x, position.y, visual.char, BackgroundFlag::None);
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

    fn draw_hp(root: &mut Root, hp: i32, max_hp: i32) {
        root.print_ex(
            1,
            root.height() - 2,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("HP: {}/{} ", hp, max_hp),
        );
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::Join;

        // Get the FOV map
        let fov_map_mutex = data.fov_map.clone();
        let fov_map = &*fov_map_mutex.lock().unwrap();

        // Get the items we'll be rendering
        let mut items = (&data.position, &data.visual, &data.entities)
            .join()
            .filter(|j| fov_map.is_in_fov(j.0.x, j.0.y))
            .collect::<Vec<_>>();
        // Things with a collider need to be drawn on top of things without a collider
        // ie. the player is drawn over a corpse
        items.sort_by(|&a, &b| {
            data.collider
                .get(a.2)
                .is_some()
                .cmp(&data.collider.get(b.2).is_some())
        });

        // Prepare for the new frame
        let ui = &mut *data.ui;
        let root = &mut ui.consoles.root;
        ui.config.apply(root);

        let map_mutex = ui.consoles.map.clone();
        let map = &mut *map_mutex.lock().unwrap();

        // Clear the screen first
        map.set_default_foreground(WHITE);
        map.clear();

        // Walls and stuff
        Self::draw_fov(map, &data.map, fov_map);

        // Monsters and stuff
        for (position, visual, _) in items {
            Self::draw_object(map, position, visual);
        }

        // Some GUI
        if let Some((living, _)) = (&data.living, &data.player).join().next() {
            Self::draw_hp(root, living.hp, living.max_hp);
        }

        // Put it all together
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
