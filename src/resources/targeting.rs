use specs::Entity;

pub struct Targeting {
    pub used_item: Entity,
    pub kind: TargetingKind,
    pub max_range: Option<f32>,
}

#[derive(PartialEq)]
pub enum TargetingKind {
    Monster,
    Tile,
}
