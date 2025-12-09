use std::fs::read_to_string;

fn main() {
    let text_file_string =
        read_to_string("./day_09/input.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_output = part_1(sample_input.clone());
    println!("Part 1: {part_1_output}");
    let part_2_output = part_2(sample_input);
    println!("Part 2: {part_2_output}");
}

fn part_1<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    let mut area: i64 = 0;
    let lines: Vec<&str> = lines.collect();
    for p in 0..lines.len() {
        for s in p..lines.len() {
            let (x1, y1) = get_point(lines[p]);
            let (x2, y2) = get_point(lines[s]);
            let x: i64 = ((x1 - x2) + 1).abs();
            let y: i64 = ((y1 - y2) + 1).abs();
            if x * y > area {
                area = x * y;
            }
        }
    }
    area
}

fn part_2<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    0
}

fn get_point(pair: &str) -> (i64, i64) {
    println!("Pair {pair}");
    let point: Vec<&str> = pair.split(",").collect();
    println!("Points {}, {}", point[0], point[1]);
    (
        point[0].trim().parse().unwrap(),
        point[1].trim().parse().unwrap(),
    )
}
