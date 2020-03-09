
use std::collections::HashSet;

pub struct Map {
    rooms: [bool; 7*7],
    edges: HashSet<(usize, usize, usize, usize)>,
}

pub fn display(map: &Map) {
    let rooms = &map.rooms;
    let mut data = String::new();
    let size: usize = 7;
    for i in 0..size {
        for k in 0..4 {
            for j in 0..size { // these rows are for the room dimensions
                if rooms[i * size + j] {
                    match k % 4 {
                        0 => { data += &" - " },
                        1 => { data += &"[ ]" },
                        2 => { data += &" - " },
                        3 => { data += if map.edges.contains(&(i, j, i+1, j)) {&" | "} else {&"   "}}  // NS corridors
                        _ => {},
                    }
                }
                // EW corridors
                data += if map.edges.contains(&(i, j, i, j+1)) && k % 3 == 1 {{&"-=-"}} else {{&"   "}};
            }
            data += &"\n";
        }
    }
    print!("{}", data);
}

pub fn test_map() -> Map {
    let mut map: Map = Map {
        rooms: [false; 7*7],
        edges: HashSet::new(),
    };
    let size = 7;
    let rooms = &mut map.rooms;
    rooms[4 * size + 5] = true;
    rooms[4 * size + 4] = true;
    rooms[4 * size + 6] = true;
    rooms[5 * size + 5] = true;
    rooms[5 * size + 4] = true;
    rooms[5 * size + 6] = true;
    rooms[6 * size + 5] = true;
    rooms[6 * size + 4] = true;
    rooms[6 * size + 6] = true;

    let edges = &mut map.edges;
    edges.insert((4,4,4,5));
    edges.insert((4,5,4,6));
    edges.insert((4,4,5,4));
    edges.insert((5,4,6,4));
    edges.insert((6,4,6,5));
    edges.insert((4,6,5,6));

    map
}