use std::{fs::read_to_string, str::Split};

fn main() {
    let text_file_string =
        read_to_string("./day_04/input.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_output = part_1(&sample_input);
    println!("Part 1: {part_1_output}")
}

fn part_1(lines: &Split<&str>) -> u32 {
 let count = 0;
 count
}

#[cfg(test)]
mod tests {
    use super::*;assert_eq!(part_1_result, 3);
    }

    #[test]
    fn part_2_test() {
        let text_file_string = read_to_string("./input_sample.txt").unwrap();
        let sample_input = text_file_string.trim().split("\n");
        let part_2_result = part_2(&sample_input);
        assert_eq!(part_2_result, 6);
    }
}
