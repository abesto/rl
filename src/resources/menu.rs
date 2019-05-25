#[derive(Clone, PartialEq)]
pub struct Menu {
    pub items: Vec<String>,
    pub header: String,
    pub width: i32,
    pub kind: MenuKind,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MenuKind {
    Inventory,
}
