use std::{fs::read_to_string, str::Split};

fn main() {
    let text_file_string =
        read_to_string("./day_02/input_sample.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split(",");
    let part_1_result = part_1(&sample_input);
    println!("Part 1: {part_1_result}")
}

fn part_1(lines: &Split<&str>) -> u64 {
    let mut result: u64 = 0;
    lines.clone().for_each(|line| {
        let mut parts = line.splitn(2, '-');
        let first: i32 = parts.next().unwrap().parse().unwrap();
        let second: i32 = parts.next().unwrap().parse().unwrap();
        println!("First {first}, Second {second}");

        for p in first..second {
            let length: u32 = p.to_string().chars().count().try_into().unwrap();
            let half: i32 = 10i32.pow(length / 2);
            // println!("length {length}, half {half}");
            // println!(
            //     "Left Half: {}, Right Half {}",
            //     p / half,
            //     (p - p - p / half).abs()
            // );
            if p / half == (p - p - p / half).abs() {
                println!("Doubles {p}");
                let s: u64 = p.try_into().expect("Error converting i32 to u64");
                result += s;
            }
        }
    });
    result
}
