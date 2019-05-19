use std::sync::{Arc, Mutex};
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

pub struct UIConfig {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

pub struct UIConsoles {
    pub root: Root,
    pub offscreen: Arc<Mutex<Offscreen>>,
}

pub struct UIState {
    pub config: UIConfig,
    pub consoles: UIConsoles,

    pub exit_requested: bool,
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

    let offscreen = Offscreen::new(config.width, config.height);

    tcod::system::set_fps(LIMIT_FPS);

    UIState {
        config,
        exit_requested: false,
        consoles: UIConsoles {
            root,
            offscreen: Arc::new(Mutex::new(offscreen)),
        },
    }
}
