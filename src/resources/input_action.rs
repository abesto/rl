#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputAction {
    Noop,

    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,

    MoveDown,

    PickUp,
    OpenDropMenu,
    OpenInventoryMenu,

    // Game operations
    NewGame,
    LoadGame,
    MainMenu,
    Exit,
    ToggleFullScreen,

    NextLevel,
}

impl Default for InputAction {
    fn default() -> Self {
        InputAction::Noop
    }
}
