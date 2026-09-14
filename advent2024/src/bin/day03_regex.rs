use regex::Regex;

fn main() {
    let input = tal::read_entire_stdin();

    println!("Part 1 solution: {}", part_one(&input));

    println!("Part 2 solution: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let pattern: Regex = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    let mut part_one_total: i32 = 0;
    for matched in pattern.captures_iter(input) {
        part_one_total += matched[1].parse::<i32>().unwrap()
            * matched[2].parse::<i32>().unwrap()
    }
    part_one_total
}

fn part_two(input: &str) -> i32 {
    let pattern: Regex =
        Regex::new(r#"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    let mut parse = true;
    let mut total = 0;
    for matched in pattern.captures_iter(input) {
        match &matched[0] {
            "do()" => {
                parse = true;
            }
            "don't()" => {
                parse = false;
            }
            _ => {
                if parse {
                    total += matched[1].parse::<i32>().unwrap()
                        * matched[2].parse::<i32>().unwrap()
                }
            }
        }
    }
    total
}
