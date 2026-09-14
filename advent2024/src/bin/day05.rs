use std::{cmp::Ordering, collections::HashMap};

fn main() {
    // k < all elements of map[k]
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut prints: Vec<Vec<u32>> = vec![];

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let rule = line.split_once('|').expect("Rule was odd");
        let parserule = (rule.0.parse().unwrap(), rule.1.parse().unwrap());

        rules.entry(parserule.0).or_default().push(parserule.1);
    }

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let print = line.split(',').map(|x| x.parse().unwrap()).collect();

        prints.push(print);
    }

    println!("Part 1 solution: {}", part_one(&rules, &prints));

    println!("Part 2 solution: {}", part_two(&rules, &prints));
}

fn part_one(rules: &HashMap<u32, Vec<u32>>, prints: &Vec<Vec<u32>>) -> u32 {
    let mut sum_of_mids = 0;
    for print in prints {
        if validate_line(rules, print) {
            sum_of_mids += print[print.len() / 2];
        }
    }

    sum_of_mids
}

fn part_two(rules: &HashMap<u32, Vec<u32>>, prints: &Vec<Vec<u32>>) -> u32 {
    let mut sum_of_mids = 0;
    for print in prints {
        if !validate_line(rules, print) {
            let print = make_valid(rules, print);
            assert!(validate_line(rules, &print));
            sum_of_mids += print[print.len() / 2];
        }
    }

    sum_of_mids
}

fn validate_line(rules: &HashMap<u32, Vec<u32>>, print: &[u32]) -> bool {
    for lhs_index in 0..print.len() - 1 {
        let lhs = print[lhs_index];
        for rhs in print[lhs_index + 1..].iter() {
            if let Some(x) = rules.get(rhs)
                && x.contains(&lhs)
            {
                return false;
            }
        }
    }
    true
}

fn make_valid(rules: &HashMap<u32, Vec<u32>>, print: &[u32]) -> Vec<u32> {
    let mut print = print.to_vec();
    print.sort_by(|lhs, rhs| {
        if let Some(x) = rules.get(lhs)
            && x.contains(rhs)
        {
            Ordering::Less
        } else if let Some(x) = rules.get(rhs)
            && x.contains(lhs)
        {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    print
}
