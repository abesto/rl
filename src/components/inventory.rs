use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, Entity, HashMapStorage,
};
use specs_derive::Component;

#[derive(Component, Clone)]
#[storage(HashMapStorage)]
pub struct Inventory(pub Vec<Entity>);

impl Inventory {
    pub fn new() -> Inventory {
        Inventory(Vec::new())
    }
}

// I wish specs_derive could do this :(

#[derive(Serialize, Deserialize)]
pub struct InventoryData<M>(pub Vec<M>);

impl<M: Marker + Serialize> ConvertSaveload<M> for Inventory
where
    for<'de> M: Deserialize<'de>,
{
    type Data = InventoryData<M>;
    type Error = NoError;

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        Ok(Inventory(
            data.0.iter().map(|id| ids(id.clone()).unwrap()).collect(),
        ))
    }

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        Ok(InventoryData(
            self.0.iter().map(|&e| ids(e).unwrap()).collect(),
        ))
    }
}
