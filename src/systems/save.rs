use std::fs::File;

use serde::{Deserialize, Serialize};
use shred_derive::SystemData;
use specs::{
    error::NoError,
    prelude::*,
    saveload::{MarkerAllocator, SerializeComponents, U64Marker, U64MarkerAllocator},
    Write,
};
use specs_derive::Component;

use crate::resources::messages::Messages;
use crate::{components::*, resources::map::Map};

#[derive(PartialEq, Serialize, Deserialize, Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Synthetic;

#[derive(SystemData)]
pub struct SavePrepSystemData<'a> {
    entity: Entities<'a>,

    map_res: ReadExpect<'a, Map>,
    map_comp: WriteStorage<'a, Map>,

    messages_res: ReadExpect<'a, Messages>,
    messages_comp: WriteStorage<'a, Messages>,

    synthetic_marker: WriteStorage<'a, Synthetic>,
    allocator: Write<'a, U64MarkerAllocator>,
    marker: WriteStorage<'a, U64Marker>,
}

pub struct SavePrepSystem;

impl<'a> System<'a> for SavePrepSystem {
    type SystemData = SavePrepSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let resources_entity = data.entity.create();
        data.synthetic_marker
            .insert(resources_entity, Synthetic)
            .unwrap();
        data.marker
            .insert(
                resources_entity,
                data.allocator.allocate(resources_entity, None),
            )
            .unwrap();
        data.map_comp
            .insert(resources_entity, data.map_res.clone())
            .unwrap();
        data.messages_comp
            .insert(resources_entity, data.messages_res.clone())
            .unwrap();
    }
}

#[derive(SystemData)]
pub struct SaveSystemData<'a> {
    entity: Entities<'a>,
    components: (
        ReadStorage<'a, Ai>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, Living>,
        ReadStorage<'a, Map>,
        ReadStorage<'a, Messages>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, PreviousPosition>,
        ReadStorage<'a, Power>,
        ReadStorage<'a, Synthetic>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Visual>,
    ),
    synthetic_marker: ReadStorage<'a, Synthetic>,
    marker: ReadStorage<'a, U64Marker>,
}

pub struct SaveSystem;

impl<'a> System<'a> for SaveSystem {
    type SystemData = SaveSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use std::io::Write;

        // Serialize
        let mut ser = ron::ser::Serializer::new(None, false);
        SerializeComponents::<NoError, U64Marker>::serialize(
            &data.components,
            &data.entity,
            &data.marker,
            &mut ser,
        )
        .unwrap();

        // Write to disk
        let mut file = File::create("savegame").unwrap();
        file.write_all(ser.into_output_string().as_bytes()).unwrap();

        // Clean any entities created by SavePrepSystem
        for (entity, _) in (&data.entity, &data.synthetic_marker).join() {
            data.entity.delete(entity).unwrap();
        }
    }
}
