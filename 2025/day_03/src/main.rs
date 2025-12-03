use std::{fs::read_to_string, str::Split};

fn main() {
    let text_file_string =
        read_to_string("./day_03/input.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_output = part_1(&sample_input);
    println!("Part 1: {part_1_output}")
}

fn part_1(lines: &Split<&str>) -> u32 {
    let mut result = 0;
    lines.clone().for_each(|line| {
        let mut maxnum = 0;
        for p in 0..line.len() - 2 {
            for s in p+1..line.len() - 1 {
                // println!(
                //     "{}{}",
                //     line.chars().nth(p).expect("Unwrap first char"),
                //     line.chars().nth(s).expect("Unwrap Second char")
                // );
                let jolt = format!(
                    "{}{}",
                    line.chars().nth(p).expect("Unwrap first char"),
                    line.chars().nth(s).expect("Unwrap Second char")
                )
                .trim()
                .parse()
                .expect("Trouble parsing concated chars");
                if jolt > maxnum {
                    maxnum = jolt
                }
            }
        }
        println!("Max Jolt: {maxnum}");
        result += maxnum;
    });
    result
}
