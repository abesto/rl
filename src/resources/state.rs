#[derive(PartialEq, Debug)]
pub enum State {
    MainMenu,
    Game,
    Loaded,
}

impl Default for State {
    fn default() -> Self {
        State::MainMenu
    }
}
