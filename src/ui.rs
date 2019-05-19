use tcod::console::*;

pub struct UiConfig {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
    pub exit_requested: bool,
}

impl UiConfig {
    pub fn new() -> UiConfig {
        UiConfig {
            width: 80,
            height: 50,
            fullscreen: false,
            exit_requested: false,
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

const LIMIT_FPS: i32 = 20;

pub fn init(config: &UiConfig) -> Root {
    use tcod::console::Root;

    let root = Root::initializer()
        .font("assets/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(config.width, config.height)
        .title("Rust/libtcod tutorial")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    root
}
