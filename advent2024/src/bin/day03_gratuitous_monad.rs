fn main() {
    let input = tal::read_entire_stdin();

    println!("Part 1 solution: {}", part_one(&input));

    println!("Part 2 solution: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    (0..input.len())
        .filter_map(|i| {
            let rest = input[i..].strip_prefix("mul(")?;
            let (lhs, rest) = rest.split_once(",")?;
            let (rhs, _) = rest.split_once(")")?;
            let lhs = lhs.parse::<i32>().ok()?;
            let rhs = rhs.parse::<i32>().ok()?;

            Some(rhs * lhs)
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let mut part_one_total: i32 = 0;
    let mut parse = true;
    for i in 0..input.len() {
        if input[i..].starts_with("do()") {
            parse = true;
            continue;
        }

        if input[i..].starts_with("don't()") {
            parse = false;
            continue;
        }

        if parse {
            let Some(rest) = input[i..].strip_prefix("mul(") else {
                continue;
            };
            let Some((lhs, rest)) = rest.split_once(",") else {
                continue;
            };
            let Some((rhs, _)) = rest.split_once(")") else {
                continue;
            };
            let Ok(lhs) = lhs.parse::<i32>() else {
                continue;
            };
            let Ok(rhs) = rhs.parse::<i32>() else {
                continue;
            };

            part_one_total += rhs * lhs;
        }
    }
    part_one_total
}
