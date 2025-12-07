use std::fs::read_to_string;

fn main() {
    let text_file_string =
        read_to_string("./day_07/input.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_output = part_1(sample_input.clone());
    println!("Part 1: {part_1_output}");
    let part_2_output = part_2(sample_input);
    println!("Part 2: {part_2_output}");
}

fn part_1<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
    let mut lines = lines.peekable();
    let mut total = 0;
    let mut linestate: Vec<bool> = vec![false; lines.peek().unwrap().len()];
    let mut nextlinestate: Vec<bool> = linestate.clone();
    for row in lines {
        for (index, col) in row.chars().enumerate() {
            println!("Index {index}, col {col}, linestate {}", linestate[index]);
            if col == 'S' {
                nextlinestate[index] = true;
            } else if col == '^' && linestate[index] {
                nextlinestate[index - 1] = true;
                nextlinestate[index + 1] = true;
                nextlinestate[index] = false;
                total += 1;
            }
        }
        linestate = nextlinestate.clone();
    }
    total
}

fn part_2<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
    0
}
