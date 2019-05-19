use std::ops::Index;

use crate::components::Position;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn floor() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

pub type Tiles = Vec<Vec<Tile>>;

pub struct Map {
    pub tiles: Tiles,
}

impl Map {
    pub fn new() -> Map {
        // fill map with "unblocked" tiles
        let mut tiles = vec![vec![Tile::floor(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        tiles[30][22] = Tile::wall();
        tiles[50][22] = Tile::wall();
        Map { tiles }
    }
}

impl Index<&Position> for Map {
    type Output = Tile;

    fn index(&self, position: &Position) -> &Tile {
        &self.tiles[position.x as usize][position.y as usize]
    }
}
