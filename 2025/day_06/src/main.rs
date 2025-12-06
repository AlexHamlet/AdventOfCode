use std::{fs::read_to_string, str::Split};

use regex::Regex;

fn main() {
    let text_file_string =
        read_to_string("./day_06/input.txt").expect("Unable to read input file.");
    let sample_input = text_file_string.trim().split("\n");
    let part_1_output = part_1(&sample_input);
    println!("Part 1: {part_1_output}");
}

fn part_1(lines: &Split<&str>) -> u64 {
    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut firstrow: bool = true;
    let mut total: u64 = 0;

    for line in lines.clone().into_iter() {
        if line.contains('+') {
            let re = Regex::new(r"\s+").unwrap();
            let preresult = re.replace_all(line, " ");
            let result = preresult.split(" ");
            for (i, sign) in result.enumerate(){
                if sign == "+"{
                    total += numbers[i].iter().sum::<u64>();
                } else if sign == "*"{
                    total += numbers[i].iter().fold(1, |accumulator, next_value| { accumulator * next_value });
                }
            }
        } else {
            let re = Regex::new(r"\s+").unwrap();
            let preresult = re.replace_all(line, " ");
            let result = preresult.trim().split(" ");
            println!("Number line Result {:?}", result);
            if firstrow == true {
                firstrow = false;
                for col in result {
                    let tempvec: Vec<u64> = vec![col.parse().unwrap()];
                    numbers.push(tempvec);
                }
                continue;
            }
            for (i, col) in result.enumerate() {
                numbers[i].push(col.parse().unwrap());
            }
        }
    }

    total
}
