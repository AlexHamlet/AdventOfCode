use tal::{Direction, Point, Tilemap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Floor,
    Obstacle,
}

fn main() {
    let mut guard_starting_pos = None;
    let area = Tilemap::new_from_char_based_stdin(|coords, ch| match ch {
        '#' => Tile::Obstacle,
        '.' => Tile::Floor,
        '^' => {
            guard_starting_pos = Some(coords);
            Tile::Floor
        }
        _ => panic!("saw a surprising tile: {ch:?}"),
    });

    let guard_starting_pos =
        guard_starting_pos.expect("didn't see a guard on that map");

    println!("Part 1 solution: {}", part_one(&area, guard_starting_pos));

    println!("Part 2 solution: {}", part_two(&area, guard_starting_pos));
}

fn part_one(area: &Tilemap<Tile>, guard_pos: Point) -> u32 {
    do_guard_logic(area, guard_pos).visit_count
}

fn part_two(area: &Tilemap<Tile>, guard_pos: Point) -> u32 {
    let mut looping_obstacle_count = 0;
    // step 1: find out which tiles the guard visited
    let potential_obstacle_map = do_guard_logic(area, guard_pos).visited_map;

    // step 2: try guard logic with a version of the map where each of those
    // tiles is replaced with an obstacle, one at a time
    for (coords, _) in potential_obstacle_map
        .coordinates()
        .zip(potential_obstacle_map.iter())
        .filter(|(coords, visited)| {
            visited.iter().any(|y| *y) && *coords != guard_pos
        })
    {
        let mut obstacle_map = area.clone();
        obstacle_map.set_tile(coords, Tile::Obstacle);
        if !do_guard_logic(&obstacle_map, guard_pos).escaped {
            looping_obstacle_count += 1;
        }
    }

    // step 3: ???
    looping_obstacle_count
}

struct GuardLogicOutput {
    /// number of tiles visited by the guard
    visit_count: u32,
    /// whether the guard escaped the map
    escaped: bool,
    /// tiles visited (and the directions they were visited while facing)
    visited_map: Tilemap<[bool; 4]>,
}

fn do_guard_logic(area: &Tilemap<Tile>, guard_pos: Point) -> GuardLogicOutput {
    let mut visit_count: u32 = 0;
    let mut guard_pos = guard_pos;
    let mut guard_facing = Direction::North;

    let mut visited_map = Tilemap::new_with_copies(
        area.get_width(),
        area.get_height(),
        [false, false, false, false],
    );

    let escaped = loop {
        let tile = area.get_tile(guard_pos).unwrap();
        assert_eq!(
            *tile,
            Tile::Floor,
            "the guard started on a {tile:?}, that's not cool",
        );

        let visited = visited_map.get_tile_mut(guard_pos).unwrap();

        if !visited[guard_facing.get_cardinal_ordinal()] {
            if visited.iter().all(|f| !*f) {
                visit_count += 1;
            }
            visited[guard_facing.get_cardinal_ordinal()] = true;
        } else {
            // we've been here (facing the same way) before
            break false;
        }

        //check in front of guard
        let next_pos = guard_pos + guard_facing;
        let next_tile = area.get_tile(next_pos);
        match next_tile {
            Some(Tile::Floor) => {
                //move foreward one
                guard_pos = next_pos;
            }
            Some(Tile::Obstacle) => {
                // rotate
                guard_facing = guard_facing.rotate_90_cw();
            }
            None => break true,
        }
    };

    GuardLogicOutput {
        visit_count,
        escaped,
        visited_map,
    }
}
