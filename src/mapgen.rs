use std::cmp;

use rand::Rng;
use specs::{
    saveload::{MarkedBuilder, U64Marker},
    world::Builder,
    World,
};
use tcod::colors;

use crate::components::*;
use crate::resources::map::*;

const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const MAX_ROOM_MONSTERS: i32 = 3;
const MAX_ROOM_ITEMS: i32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

pub fn create_room(room: Rect, map: &mut Tiles) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::floor();
        }
    }
}

fn place_objects(map: &Map, room: Rect, world: &mut World) {
    // choose random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

    for _ in 0..num_monsters {
        // choose random spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        let position = Position { x, y };

        // Check that we're not trying to place this monster at a location already occupied
        // by something.
        if map.is_blocked(&position, &mut *world) {
            continue;
        }

        if rand::random::<f32>() < 0.8 {
            // 80% chance of getting an orc
            // create an orc
            world
                .create_entity()
                .with(position)
                .with(Velocity::new())
                .with(Visual {
                    char: 'o',
                    color: colors::DESATURATED_GREEN,
                    always_visible: false,
                })
                .with(Collider::new())
                .with(Name::new("orc"))
                .with(Living {
                    alive: true,
                    max_hp: 10,
                    hp: 10,
                    defense: 0,
                })
                .with(Power(3))
                .with(Ai::Basic)
                .with(Action::noop())
                .with(Energy::new())
                .marked::<U64Marker>()
                .build();
        } else {
            world
                .create_entity()
                .with(position)
                .with(Velocity::new())
                .with(Visual {
                    char: 'T',
                    color: colors::DARKER_GREEN,
                    always_visible: false,
                })
                .with(Collider::new())
                .with(Name::new("troll"))
                .with(Living {
                    alive: true,
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                })
                .with(Power(4))
                .with(Ai::Basic)
                .with(Energy::new())
                .with(Action::noop())
                .marked::<U64Marker>()
                .build();
        };
    }

    // choose random number of items
    let num_items = rand::thread_rng().gen_range(0, MAX_ROOM_ITEMS + 1);

    for _ in 0..num_items {
        // choose random spot for this item
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        let position = Position { x, y };

        // only place it if the tile is not blocked
        if !map.is_blocked(&position, &mut *world) {
            let dice = rand::random::<f32>();
            if dice < 0.7 {
                // create a healing potion (70% chance)
                world
                    .create_entity()
                    .with(position)
                    .with(Visual {
                        char: '!',
                        color: colors::VIOLET,
                        always_visible: false,
                    })
                    .with(Name::new("healing potion"))
                    .with(Item::Heal)
                    .marked::<U64Marker>()
                    .build();
            } else if dice < 0.7 + 0.1 {
                // create a lightning bolt scroll (10% chance)
                world
                    .create_entity()
                    .with(position)
                    .with(Visual {
                        char: '#',
                        color: colors::LIGHT_YELLOW,
                        always_visible: false,
                    })
                    .with(Name::new("scroll of lightning bolt"))
                    .with(Item::Lightning)
                    .marked::<U64Marker>()
                    .build();
            } else if dice < 0.7 + 0.1 + 0.1 {
                // create a lightning bolt scroll (10% chance)
                world
                    .create_entity()
                    .with(position)
                    .with(Visual {
                        char: '#',
                        color: colors::LIGHT_BLUE,
                        always_visible: false,
                    })
                    .with(Name::new("scroll of confusion"))
                    .with(Item::Confuse)
                    .marked::<U64Marker>()
                    .build();
            } else {
                // create a fireball scroll (10% chance)
                world
                    .create_entity()
                    .with(position)
                    .with(Visual {
                        char: '#',
                        color: colors::DARK_RED,
                        always_visible: false,
                    })
                    .with(Name::new("scroll of fireball"))
                    .with(Item::Fireball)
                    .marked::<U64Marker>()
                    .build();
            }
        }
    }
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Tiles) {
    for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
        map[x as usize][y as usize] = Tile::floor();
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Tiles) {
    for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
        map[x as usize][y as usize] = Tile::floor();
    }
}

pub fn generate_map(world: &mut World) {
    let mut map = Map {
        tiles: vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
        spawn_point: Position { x: 0, y: 0 },
    };
    let mut rooms = vec![];

    for _ in 0..MAX_ROOMS {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        // run through the other rooms and see if they intersect with this one
        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            // this means there are no intersections, so this room is valid

            // "paint" it to the map's tiles
            create_room(new_room, &mut map.tiles);
            // add some content to this room, such as monsters
            place_objects(&map, new_room, world);

            // center coordinates of the new room, will be useful later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // this is the first room, where the player starts at
                map.spawn_point.x = new_x;
                map.spawn_point.y = new_y;
            } else {
                // all rooms after the first:
                // connect it to the previous room with a tunnel

                // center coordinates of the previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                // draw a coin (random bool value -- either true or false)
                if rand::random() {
                    // first move horizontally, then vertically
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map.tiles);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map.tiles);
                } else {
                    // first move vertically, then horizontally
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map.tiles);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map.tiles);
                }
            }

            // finally, append the new room to the list
            rooms.push(new_room);
        }
    }

    // create stairs at the center of the last room
    let (last_room_x, last_room_y) = rooms[rooms.len() - 1].center();
    world
        .create_entity()
        .with(Position {
            x: last_room_x,
            y: last_room_y,
        })
        .with(Visual {
            char: '>',
            color: colors::WHITE,
            always_visible: true,
        })
        .with(Name::new("stairs"))
        .marked::<U64Marker>()
        .build();

    world.add_resource(map);
}
