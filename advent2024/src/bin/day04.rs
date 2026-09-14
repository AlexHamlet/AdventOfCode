use tal::{Direction, Point, Tilemap};

fn main() {
    let map = Tilemap::new_from_char_based_stdin(|_, ch| ch);

    println!("Part 1 solution: {}", part_one(&map));

    println!("Part 2 solution: {}", part_two(&map));
}

fn part_one(map: &Tilemap<char>) -> usize {
    let mut xmascount = 0;

    for (coords, letter) in map.coordinates().zip(map.iter()) {
        if *letter == 'X' {
            xmascount += count_directions(coords, map)
        }
    }
    xmascount
}

fn part_two(map: &Tilemap<char>) -> usize {
    let mut xmascount = 0;

    for (coords, letter) in map.coordinates().zip(map.iter()) {
        if *letter == 'A' && count_directions_part_two(coords, map) {
            xmascount += 1;
        }
    }
    xmascount
}

fn count_directions(coords: Point, map: &Tilemap<char>) -> usize {
    assert_eq!(map.get_tile(coords), Some(&'X'));
    Direction::all()
        .filter(|dir| {
            map.get_tile(coords + *dir * 1) == Some(&'M')
                && map.get_tile(coords + *dir * 2) == Some(&'A')
                && map.get_tile(coords + *dir * 3) == Some(&'S')
        })
        .count()
}

fn count_directions_part_two(coords: Point, map: &Tilemap<char>) -> bool {
    assert_eq!(map.get_tile(coords), Some(&'A'));
    Direction::diagonals().all(|dir| {
        (map.get_tile(coords + dir * 1) == Some(&'M')
            && map.get_tile(coords + dir * -1) == Some(&'S'))
            || (map.get_tile(coords + dir * 1) == Some(&'S')
                && map.get_tile(coords + dir * -1) == Some(&'M'))
    })
}
