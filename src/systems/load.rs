use std::fs::File;

use shred_derive::SystemData;
use specs::{
    error::NoError,
    prelude::*,
    saveload::{DeserializeComponents, U64Marker, U64MarkerAllocator},
};

use crate::{
    components::*, resources::map::Map, resources::messages::Messages, systems::save::Synthetic,
};

#[derive(SystemData)]
pub struct LoadSystemData<'a> {
    entity: Entities<'a>,
    components: (
        WriteStorage<'a, Ai>,
        WriteStorage<'a, Collider>,
        WriteStorage<'a, Inventory>,
        WriteStorage<'a, Item>,
        WriteStorage<'a, Living>,
        WriteStorage<'a, Map>,
        WriteStorage<'a, Messages>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, PreviousPosition>,
        WriteStorage<'a, Power>,
        WriteStorage<'a, Synthetic>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Visual>,
    ),

    allocator: Write<'a, U64MarkerAllocator>,
    marker: WriteStorage<'a, U64Marker>,

    map_res: WriteExpect<'a, Map>,
    messages_res: WriteExpect<'a, Messages>,
}

pub struct LoadSystem;

impl<'a> System<'a> for LoadSystem {
    type SystemData = LoadSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let file = File::open("savegame").unwrap();
        let mut ser = serde_json::Deserializer::from_reader(file);
        DeserializeComponents::<Combined, _>::deserialize(
            &mut data.components,
            &data.entity,
            &mut data.marker,
            &mut data.allocator,
            &mut ser,
        )
        .unwrap();

        // Pull in global stuff from the synthetic entity they were saved onto, and clean them up
        // from the world space
        for (entity, map, messages) in (&data.entity, &data.components.5, &data.components.6).join()
        {
            *data.map_res = map.clone();
            *data.messages_res = messages.clone();
            data.entity.delete(entity).unwrap();
        }
    }
}

// DeserializeComponents needs all this for whatever reason

#[derive(Debug)]
enum Combined {
    SerdeJson(serde_json::Error),
}

// Implementing the required `Display`-trait, by matching the `Combined` enum, allowing different error types to be displayed.
impl std::fmt::Display for Combined {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Combined::SerdeJson(ref e) => write!(f, "{}", e),
        }
    }
}

// This returns the `serde::Error` in form of the `Combined` enum, which can then be matched and displayed accordingly.
impl From<serde_json::Error> for Combined {
    fn from(x: serde_json::Error) -> Self {
        Combined::SerdeJson(x)
    }
}

// This cannot be called.
impl From<NoError> for Combined {
    fn from(e: NoError) -> Self {
        match e {}
    }
}
