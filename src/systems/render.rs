use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Mouse;
use tcod::map::Map as FovMap;

use crate::components::*;
use crate::resources::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use crate::resources::menu::Menu;
use crate::resources::messages::Messages;
use crate::resources::ui::{
    UIState, BAR_WIDTH, PANEL_HEIGHT, PANEL_Y, SCREEN_HEIGHT, SCREEN_WIDTH,
};

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

const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

pub struct RenderSystem;

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    collider: ReadStorage<'a, Collider>,
    living: ReadStorage<'a, Living>,
    name: ReadStorage<'a, Name>,
    player: ReadStorage<'a, Player>,
    position: ReadStorage<'a, Position>,
    visual: ReadStorage<'a, Visual>,

    entities: Entities<'a>,

    fov_map: ReadExpect<'a, Arc<Mutex<FovMap>>>,
    map: ReadExpect<'a, Map>,
    messages: ReadExpect<'a, Messages>,
    mouse: ReadExpect<'a, Mouse>,
    ui: WriteExpect<'a, UIState>,
    menu: ReadExpect<'a, Option<Menu>>,
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

    fn draw_hp(panel: &mut Offscreen, hp: i32, max_hp: i32) {
        // prepare to render the GUI panel
        panel.set_default_background(BLACK);
        panel.clear();

        // show the player's stats
        Self::render_bar(
            panel, 1, 1, BAR_WIDTH, "HP", hp, max_hp, LIGHT_RED, DARKER_RED,
        );
    }

    #[allow(clippy::too_many_arguments)]
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

    fn render_messages(panel: &mut Offscreen, messages: &[(String, Color)]) {
        // print the game messages, one line at a time
        let mut y = MSG_HEIGHT as i32;
        for &(ref msg, color) in messages.iter().rev() {
            let msg_height = panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
            y -= msg_height;
            if y < 0 {
                break;
            }
            panel.set_default_foreground(color);
            panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        }
    }

    fn render_names_under_mouse(panel: &mut Offscreen, names: &[String]) {
        panel.set_default_foreground(LIGHT_GREY);
        panel.print_ex(
            1,
            0,
            BackgroundFlag::None,
            TextAlignment::Left,
            names.join(", "),
        );
    }

    fn render_menu(root: &mut Root, menu: &Menu) {
        assert!(
            menu.items.len() <= 26,
            "Cannot have a menu with more than 26 options."
        );

        // calculate total height for the header (after auto-wrap) and one line per option
        let header_height = root.get_height_rect(0, 0, menu.width, SCREEN_HEIGHT, &menu.header);
        let height = menu.items.len() as i32 + header_height;

        // create an off-screen console that represents the menu's window
        let mut window = Offscreen::new(menu.width, height);

        // print the header, with auto-wrap
        window.set_default_foreground(WHITE);
        window.print_rect_ex(
            0,
            0,
            menu.width,
            height,
            BackgroundFlag::None,
            TextAlignment::Left,
            &menu.header,
        );

        // print all the options
        for (index, option_text) in menu.items.iter().enumerate() {
            let menu_letter = (b'a' + index as u8) as char;
            let text = format!("({}) {}", menu_letter, option_text);
            window.print_ex(
                0,
                header_height + index as i32,
                BackgroundFlag::None,
                TextAlignment::Left,
                text,
            );
        }

        // blit the contents of "window" to the root console
        let x = SCREEN_WIDTH / 2 - menu.width / 2;
        let y = SCREEN_HEIGHT / 2 - height / 2;
        tcod::console::blit(
            &mut window,
            (0, 0),
            (menu.width, height),
            root,
            (x, y),
            1.0,
            0.7,
        );

        // present the root console to the player and wait for a key-press
        root.flush();
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // Get the FOV map
        let fov_map_mutex = data.fov_map.clone();
        let fov_map = &*fov_map_mutex.lock().unwrap();

        // Get the items we'll be rendering
        let mut items = (&data.position, &data.visual, &data.entities, &data.name)
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
        for (position, visual, _, _) in &items {
            Self::draw_object(map, position, visual);
        }

        // Blit the map
        blit(
            &*map,
            (0, 0),
            (MAP_WIDTH, MAP_HEIGHT),
            root,
            (0, 0),
            1.0,
            1.0,
        );

        // Some GUI
        let panel = &mut consoles.panel;
        if let Some((living, _)) = (&data.living, &data.player).join().next() {
            Self::draw_hp(panel, living.hp, living.max_hp);
        }
        Self::render_messages(panel, &(*data.messages).inner);

        // Mouse look
        let mouse_pos = Position {
            x: data.mouse.cx as i32,
            y: data.mouse.cy as i32,
        };
        Self::render_names_under_mouse(
            panel,
            &items
                .iter()
                .filter(|j| j.0 == &mouse_pos)
                .map(|j| (j.3).0.clone())
                .collect::<Vec<_>>(),
        );

        // Blit the GUI
        blit(
            panel,
            (0, 0),
            (SCREEN_WIDTH, PANEL_HEIGHT),
            root,
            (0, PANEL_Y),
            1.0,
            1.0,
        );

        // And now the menu, if we have one
        if let Some(menu) = &*data.menu {
            Self::render_menu(root, &menu);
        }

        root.flush();
    }
}
