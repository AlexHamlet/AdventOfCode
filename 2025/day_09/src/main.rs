use std::fs::read_to_string;

fn main() {
    let text_file_string =
        read_to_string("./day_09/input_sample.txt").expect("Unable to read input file.");
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
    let mut area: i64 = 0;
    let lines: Vec<&str> = lines.collect();
    for p in 0..lines.len() {
        for s in p..lines.len() {
            let (x1, y1) = get_point(lines[p]);
            let (x2, y2) = get_point(lines[s]);
            let min_x = get_min(x1, x2);
            let min_y = get_min(y1, y2);
            let max_x = get_max(x1, x2);
            let max_y = get_max(y1, y2);
            let mut isvalid = true;

            for t in 0..lines.len() {
                if p == t || p == s {
                    continue;
                }
                let (x3, y3) = get_point(lines[t]);
                let x_check = x3 > min_x && x3 < max_x;
                let y_check = y3 > min_y && y3 < max_y;

                if x_check || y_check {
                    isvalid = false;
                    break;
                }
            }

            let x: i64 = ((x1 - x2) + 1).abs();
            let y: i64 = ((y1 - y2) + 1).abs();
            if x * y > area && isvalid {
                area = x * y;
            }

            // 'inner: for forwards_check in 0..pairs.len() - 1 {
            //     let first = pairs[forwards_check];
            //     let second = pairs[forwards_check + 1];
            //     let left =
            //         first.0 <= min_x && second.0 > min_x && first.1 > min_y && first.1 < max_y;
            //     let right =
            //         first.0 >= max_x && second.0 < max_x && first.1 > min_y && first.1 < max_y;
            //     let down =
            //         first.1 <= min_y && second.1 > min_y && first.0 > min_x && first.0 < max_x;
            //     let up = first.1 >= max_y && second.1 < max_y && first.0 > min_x && first.0 < max_x;

            //     if left || right || up || down {
            //         good = false;
            //         break 'inner;
            //     }
            // }
        }
    }
    area
}

fn get_point(pair: &str) -> (i64, i64) {
    let point: Vec<&str> = pair.split(",").collect();
    (
        point[0].trim().parse().unwrap(),
        point[1].trim().parse().unwrap(),
    )
}

fn get_min(x1: i64, x2: i64) -> i64 {
    if x1 > x2 {
        return x1;
    }
    x2
}

fn get_max(x1: i64, x2: i64) -> i64 {
    if x1 > x2 {
        return x2;
    }
    x1
}
