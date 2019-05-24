use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::map::Map as FovMap;

use crate::resources::map::{Map, MAP_HEIGHT, MAP_WIDTH};

#[derive(SystemData)]
pub struct FogOfWarSystemData<'a> {
    fov_map: ReadExpect<'a, Arc<Mutex<FovMap>>>,
    map: WriteExpect<'a, Map>,
}

pub struct FogOfWarSystem;

impl<'a> System<'a> for FogOfWarSystem {
    type SystemData = FogOfWarSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let fov_map_mutex = data.fov_map.clone();
        let fov_map = fov_map_mutex.lock().unwrap();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if fov_map.is_in_fov(x, y) {
                    data.map.tiles[x as usize][y as usize].explored = true;
                }
            }
        }
    }
}
