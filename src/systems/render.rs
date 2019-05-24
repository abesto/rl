use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;

use crate::components::*;
use crate::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::ui::{BAR_WIDTH, PANEL_HEIGHT, PANEL_Y, SCREEN_WIDTH};

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

    fn draw_hp(root: &mut Root, panel: &mut Offscreen, hp: i32, max_hp: i32) {
        // prepare to render the GUI panel
        panel.set_default_background(BLACK);
        panel.clear();

        // show the player's stats
        Self::render_bar(
            panel, 1, 1, BAR_WIDTH, "HP", hp, max_hp, LIGHT_RED, DARKER_RED,
        );

        // blit the contents of `panel` to the root console
        blit(
            panel,
            (0, 0),
            (SCREEN_WIDTH, PANEL_HEIGHT),
            root,
            (0, PANEL_Y),
            1.0,
            1.0,
        );
    }

    fn render_bar(
        panel: &mut Offscreen,
        x: i32,
        y: i32,
        total_width: i32,
        name: &str,
        value: i32,
        maximum: i32,
        bar_color: Color,
        back_color: Color,
    ) {
        // render a bar (HP, experience, etc). First calculate the width of the bar
        let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

        // render the background first
        panel.set_default_background(back_color);
        panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

        // now render the bar on top
        panel.set_default_background(bar_color);
        if bar_width > 0 {
            panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
        }

        // finally, some centered text with the values
        panel.set_default_foreground(WHITE);
        panel.print_ex(
            x + total_width / 2,
            y,
            BackgroundFlag::None,
            TextAlignment::Center,
            &format!("{}: {}/{}", name, value, maximum),
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
        let consoles_mutex = ui.consoles.clone();
        let consoles = &mut *consoles_mutex.lock().unwrap();
        let root = &mut consoles.root;
        ui.config.apply(root);

        let map = &mut consoles.map;

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
            Self::draw_hp(root, &mut consoles.panel, living.hp, living.max_hp);
        }

        // Put it all together
        blit(
            &*map,
            (0, 0),
            (MAP_WIDTH, MAP_HEIGHT),
            root,
            (0, 0),
            1.0,
            1.0,
        );

        root.flush();
    }
}
