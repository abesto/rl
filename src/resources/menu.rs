pub struct Menu {
    pub items: Vec<String>,
    pub header: String,
    pub width: i32,
    pub kind: MenuKind,
}

#[derive(Clone, PartialEq)]
pub enum MenuKind {
    Inventory,
}
