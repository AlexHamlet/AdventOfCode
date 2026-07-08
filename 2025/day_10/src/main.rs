use std::fs::read_to_string;

fn main() {
    let text_file_string =
        read_to_string("./day_10/input_sample.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_output = part_1(sample_input.clone());
    println!("Part 1: {part_1_output}");
    let part_2_output = part_2(sample_input);
    println!("Part 2: {part_2_output}");
}

fn part_1<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    let mut press_total = 0;

    for line in lines{
        let (expected_lights, buttons, _joltage) = parse_line(line);
    }

    press_total
}

fn part_2<'a>(lines: impl Iterator<Item = &'a str>) -> i64 {
    0
}

fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<u32>>, Vec<u32>) {
    let line: Vec<&str> = line.split(" ").collect();

    let mut lights: Vec<bool> = vec![];
    for light in &line[0]{
        if *light == "."{
            lights.push(false);
        }
        if *light == "#"{
            lights.push(true);
        }
    }

    let buttons: Vec<Vec<u32>> = vec![];
    for button in &line[1..line.len() - 1]{
        let mut values = vec![];
        for val in button{
            values.push(0);
        }
        buttons.push(values);
    }

    // let joltage = line[line.len()];

    (lights, buttons, vec![])
}
