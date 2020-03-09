use std::collections::HashSet;
use crate::room::{Room, Door, DoorState};

const MAP_SIZE: usize = 7;

pub struct Map {
    rooms: [bool; MAP_SIZE * MAP_SIZE],
    edges: HashSet<(usize, usize, usize, usize)>,
}

pub fn display(map: &Map) {
    let rooms = &map.rooms;
    let mut data = String::new();
    for i in 0..MAP_SIZE {
        for k in 0..4 {
            for j in 0..MAP_SIZE { // these rows are for the room dimensions
                if rooms[i * MAP_SIZE + j] {
                    match k % 4 {
                        0 => { data += &" - " },
                        1 => { data += &"[ ]" },
                        2 => { data += &" - " },
                        3 => { data += if map.edges.contains(&(j, i, j, i+1)) {&" | "} else {&"   "}}  // NS corridors
                        _ => {},
                    }
                }
                // EW corridors
                data += if map.edges.contains(&(j, i, j+1, i)) && k % 3 == 1 {{&"-=-"}} else {{&"   "}};
            }
            data += &"\n";
        }
    }
    print!("{}", data);
}

pub fn add_room(room: &Room, map: &mut Map, x: usize, y: usize) {
    map.rooms[x * MAP_SIZE + y] = true;
    for door in room.doors.iter() {
        match door {
            Door::North(DoorState::Open) => {map.edges.insert((x, y, x - 1, y));},
            Door::South(DoorState::Open) => {map.edges.insert((x, y, x + 1, y));},
            Door::East(DoorState::Open) => {map.edges.insert((x, y, x, y - 1));},
            Door::West(DoorState::Open) => {map.edges.insert((x, y, x, y + 1));},
            _ => {},
        }
    }
}

pub fn test_map() -> Map {
    let mut map: Map = Map {
        rooms: [false; MAP_SIZE * MAP_SIZE],
        edges: HashSet::new(),
    };

    add_room(&Room::new(60), &mut map, 0, 0);
    add_room(&Room::new(60), &mut map, 0, 1);
    add_room(&Room::new(60), &mut map, 0, 2);
    add_room(&Room::new(60), &mut map, 1, 0);
    add_room(&Room::new(60), &mut map, 1, 1);
    add_room(&Room::new(60), &mut map, 1, 2);
    add_room(&Room::new(60), &mut map, 2, 0);
    add_room(&Room::new(60), &mut map, 2, 1);
    add_room(&Room::new(60), &mut map, 2, 2);

    map
}