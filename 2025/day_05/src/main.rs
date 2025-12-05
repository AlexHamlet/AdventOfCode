use std::{fs::read_to_string, str::Split};

fn main() {
    let text_file_string =
        read_to_string("./day_05/input.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_output = part_1(&sample_input);
    println!("Part 1: {part_1_output}");
    let part_2_output = part_2(&sample_input);
    println!("Part 2: {part_2_output}")
}

fn part_1(lines: &Split<&str>) -> u32 {
    let mut count = 0;
    let (ranges, ingredients) = parse_input(lines);

    for ingredient in ingredients {
        if search_ranges(ingredient, &ranges) {
            count += 1
        }
    }

    count
}

fn part_2(lines: &Split<&str>) -> u64 {
    let mut count = 0;
    let (ranges, ingredients) = parse_input(lines);

    let simplified_ranges = simplify_ranges(ranges);

    for range in simplified_ranges {
        count += range.1 - range.0 + 1;
    }

    count
}

fn simplify_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        }

        merged.push((start, end));
    }

    merged
}

fn search_ranges(num: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for range in ranges {
        if num >= range.0 && num <= range.1 {
            return true;
        }
    }
    false
}

fn parse_input(lines: &Split<&str>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];

    let mut isRanges = true;
    for mut line in lines.clone().into_iter() {
        line = line.trim();
        if line.is_empty() {
            isRanges = false;
            continue;
        }

        if isRanges {
            let mut pieces = line.split('-');
            let start: u64 = pieces.next().unwrap().parse().expect("Error parsing start");
            let end: u64 = pieces.next().unwrap().parse().expect("Error parsing end");
            ranges.push((start, end));
        } else {
            ingredients.push(line.parse().expect("Error parsing ingredients"));
        }
    }

    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;
    assert_eq!(part_1_result, 3);

    #[test]
    fn part_1_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_1_result = part_1(&sample_input);
        assert_eq!(part_1_result, 3);
    }
}
