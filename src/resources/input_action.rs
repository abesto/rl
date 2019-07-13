#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputAction {
    Noop,

    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,

    MoveDown,

    PickUp,
    Drop(usize),
    UseFromInventory(usize),

    OpenDropMenu,
    OpenInventoryMenu,
    MenuChoice(usize),
    DismissMenu,

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
