use std::sync::{Arc, Mutex};

use shred_derive::SystemData;
use specs::prelude::*;
use tcod::map::{FovAlgorithm, Map as FovMap};

use crate::components::{Player, Position, PreviousPosition};
use crate::map::{Tiles, MAP_HEIGHT, MAP_WIDTH};

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

#[derive(SystemData)]
pub struct FovSystemData<'a> {
    player: ReadStorage<'a, Player>,
    prev_position: ReadStorage<'a, PreviousPosition>,
    position: ReadStorage<'a, Position>,

    fov_map: WriteExpect<'a, Arc<Mutex<FovMap>>>,
}

pub struct FovSystem;

impl<'a> System<'a> for FovSystem {
    type SystemData = FovSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        for (prev, pos, _) in (&data.prev_position, &data.position, &data.player).join() {
            // recompute FOV if needed (the player moved or something)
            if prev.x != pos.x || prev.y != pos.x {
                let fov_map_mutex = data.fov_map.clone();
                let fov_map = &mut *fov_map_mutex.lock().unwrap();
                fov_map.compute_fov(pos.x, pos.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
            }
        }
    }
}

pub fn new_fov_map(map: &Tiles) -> Arc<Mutex<FovMap>> {
    let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            fov_map.set(
                x,
                y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked,
            );
        }
    }
    Arc::new(Mutex::new(fov_map))
}
