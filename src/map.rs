use std::ops::Index;

use crate::components::{Collider, Position};
use specs::join::JoinIter;
use specs::{ReadStorage, World};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
}

impl Tile {
    pub fn floor() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            explored: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            explored: false,
        }
    }
}

pub type Tiles = Vec<Vec<Tile>>;

pub struct Map {
    pub tiles: Tiles,
    pub spawn_point: Position,
}

impl Map {
    #[allow(dead_code)]
    pub fn new_simple() -> Map {
        use crate::mapgen::*;
        // fill map with "unblocked" tiles
        let mut tiles = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        // Carve out a couple of rooms
        create_room(Rect::new(20, 15, 10, 15), &mut tiles);
        create_room(Rect::new(50, 15, 10, 15), &mut tiles);
        create_h_tunnel(25, 55, 23, &mut tiles);

        Map {
            tiles,
            spawn_point: Position { x: 25, y: 23 },
        }
    }

    pub fn new_random(world: &mut World) {
        crate::mapgen::generate_map(world)
    }

    pub fn is_blocked(
        &self,
        pos: &Position,
        join: JoinIter<(&ReadStorage<Position>, &ReadStorage<Collider>)>,
    ) -> bool {
        // First check for walls
        if self[pos].blocked {
            return true;
        }
        // Check for objects blocking movement
        for (collider_pos, _) in join {
            if collider_pos == pos {
                return true;
            }
        }
        false
    }
}

impl Index<&Position> for Map {
    type Output = Tile;

    fn index(&self, position: &Position) -> &Tile {
        &self.tiles[position.x as usize][position.y as usize]
    }
}
