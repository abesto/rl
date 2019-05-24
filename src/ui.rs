use std::sync::{Arc, Mutex};
use tcod::console::*;

pub const BAR_WIDTH: i32 = 20;
pub const PANEL_HEIGHT: i32 = 7;
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = crate::map::MAP_HEIGHT + PANEL_HEIGHT;
pub const LIMIT_FPS: i32 = 20;
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

pub struct UIConfig {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

pub struct UIConsoles {
    pub root: Root,
    pub map: Offscreen,
    pub panel: Offscreen,
}

pub struct UIState {
    pub config: UIConfig,
    pub consoles: Arc<Mutex<UIConsoles>>,
}

impl UIConfig {
    pub fn new() -> UIConfig {
        UIConfig {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            fullscreen: false,
        }
    }

    pub fn apply(&self, root: &mut Root) {
        if root.width() != self.width {
            panic!("Don't know how to change width at runtime");
        }
        if root.height() != self.height {
            panic!("Don't know how to change the height at runtime");
        }
        if root.is_fullscreen() != self.fullscreen {
            root.set_fullscreen(self.fullscreen);
        }
    }
}

pub fn init(config: UIConfig) -> UIState {
    use tcod::console::Root;

    let root = Root::initializer()
        .font("assets/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config.width, config.height)
        .title("Rust/libtcod tutorial")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    UIState {
        config,
        consoles: Arc::new(Mutex::new(UIConsoles {
            root,
            map: Offscreen::new(crate::map::MAP_WIDTH, crate::map::MAP_HEIGHT),
            panel: Offscreen::new(
                crate::map::MAP_WIDTH,
                SCREEN_HEIGHT - crate::map::MAP_HEIGHT,
            ),
        })),
    }
}
