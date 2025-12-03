use std::fs::read_to_string;
use std::path::absolute;
use std::str::Split;

fn main() {
    let text_file_string =
        read_to_string("./day_01/input.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_result = part_1(&sample_input);
    let part_2_result = part_2(&sample_input);
    println!("Part One Result {part_1_result}");
    println!("Part Two Result {part_2_result}");
}

fn part_1(lines: &Split<&str>) -> u32 {
    let mut dial_position: i32 = 50;
    let mut count: u32 = 0;
    lines.clone().for_each(|line| {
        dial_position += parse_line(line);
        dial_position = clamp(dial_position);
        if dial_position == 0 {
            count += 1;
        }
    });

    count
}

fn part_2(lines: &Split<&str>) -> i32 {
    let mut dial_position: i32 = 50;
    let mut count: i32 = 0;
    lines.clone().for_each(|line| {
        println!("1:Dial Pos {dial_position}, Count {count}");
        let dist = parse_line(line);
        for p in 0..dist {
            dial_position += 1;
            dial_position = clamp(dial_position);
            if dial_position == 0 {
                count += 1;
            }
        }
        for p in dist..0 {
            dial_position -= 1;
            dial_position = clamp(dial_position);
            if dial_position == 0 {
                count += 1;
            }
        }
        println!("2:Dial Pos {dial_position}, Count {count}")
    });

    count
}

fn parse_line(line: &str) -> i32 {
    let dir = line.chars().nth(0).unwrap();
    let dist: &str = &line[1..].trim();
    println!("Dir {dir}, Dist {dist}");
    let mut numdist: i32 = dist.parse().expect("Error parsing distance.");
    if dir == 'L' {
        numdist *= -1;
    }
    numdist
}

fn detect_overflow(num: i32) -> bool {
    num <= 0 || num > 99
}

fn clamp(num: i32) -> i32 {
    let mut num = num % 100;
    num += 100;
    num % 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_test_positive() {
        assert_eq!(clamp(250), 50)
    }
    #[test]
    fn clamp_test_negative() {
        assert_eq!(clamp(-125), 75)
    }
    #[test]
    fn clamp_test_within_range() {
        assert_eq!(clamp(80), 80)
    }
    #[test]
    fn parse_line_test_clockwise() {
        assert_eq!(parse_line("R36"), 36)
    }
    #[test]
    fn parse_line_test_counterclockwise() {
        assert_eq!(parse_line("L83"), -83)
    }
    #[test]
    fn part_1_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_1_result = part_1(&sample_input);
        assert_eq!(part_1_result, 3);
    }

    #[test]
    fn part_2_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_2_result = part_2(&sample_input);
        assert_eq!(part_2_result, 6);
    }
}
