use std::fs::read_to_string;

fn main() {
    let text_file_string =
        read_to_string("./day_07/input_sample.txt").expect("Unable to read input file.");
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
            if col == 'S' {
                nextlinestate[index] = true;
            } else if col == '^' && linestate[index] {
                nextlinestate[index - 1] = true;
                nextlinestate[index + 1] = true;
                nextlinestate[index] = false;
                total += 1;
            }
        }
        //linestate = nextlinestate.clone();
        // premature optimization that breaks it:
        //std::mem::swap(&mut linestate, &mut nextlinestate);
        // postmature optimization:
        linestate[..].copy_from_slice(&nextlinestate[..]);
        /*
        // logically speaking, these do the same thing:
        for (index, state) in nextlinestate.iter().copied().enumerate() {
            linestate[index] = state;
        }
        for (state, newstate) in linestate.iter_mut().zip(newlinestate.iter()) {
            *state = *newstate;
        }
        */
    }
    total
}

fn part_2<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
    let lines: Vec<&'a str> = lines.collect();
    let mut solution_index = 0;
    let mut linestate: Vec<u64> = vec![1; lines[0].len()];
    for row in lines.iter().rev() {
        for (index, col) in row.chars().enumerate() {
            if col == 'S' {
                solution_index = index;
            } else if col == '^' {
                linestate[index] = linestate[index + 1] + linestate[index - 1]
            }
        }
    }
    linestate[solution_index]
}
