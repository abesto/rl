use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::map::Map as FovMap;

use crate::resources::{
    map::{Map, MAP_HEIGHT, MAP_WIDTH},
    state::State,
};

#[derive(SystemData)]
pub struct FogOfWarSystemData<'a> {
    fov_map: Option<ReadExpect<'a, Arc<Mutex<FovMap>>>>,
    map: Option<WriteExpect<'a, Map>>,
    state: ReadExpect<'a, State>,
}

pub struct FogOfWarSystem;

impl<'a> System<'a> for FogOfWarSystem {
    type SystemData = FogOfWarSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.state != State::Game {
            return;
        }
        let fov_map_mutex = data.fov_map.as_ref().unwrap().clone();
        let fov_map = fov_map_mutex.lock().unwrap();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if fov_map.is_in_fov(x, y) {
                    data.map.as_mut().unwrap().tiles[x as usize][y as usize].explored = true;
                }
            }
        }
    }
}
