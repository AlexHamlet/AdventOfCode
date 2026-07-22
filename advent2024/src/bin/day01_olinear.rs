use std::collections::HashMap;

fn main() {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (lhs, rhs) = line.split_once("   ").unwrap();
        let lhs: usize = lhs.parse().unwrap();
        let rhs: usize = rhs.parse().unwrap();
        list1.push(lhs);
        list2.push(rhs);
    }

    list1.sort();
    list2.sort();

    let mut count = 0;
    for (lhs, rhs) in list1.iter().zip(list2.iter()) {
        count += lhs.abs_diff(*rhs);
    }

    println!("Part 1 solution: {count}");

    let mut list2populations: HashMap<usize, usize> = HashMap::new();
    for p in list2 {
        *(list2populations.entry(p).or_default()) += 1;
    }

    let mut answer2 = 0;
    for lhs in list1.iter() {
        answer2 += lhs * list2populations.get(lhs).copied().unwrap_or(0);
    }

    println!("Part 2 solution: {answer2}");
}
