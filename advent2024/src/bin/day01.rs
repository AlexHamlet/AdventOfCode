use std::collections::HashMap;

use tal::CountDupes;

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
    /*
    let mut count = 0;
    for (lhs, rhs) in list1.iter().zip(list2.iter()) {
        count += lhs.abs_diff(*rhs);
    }
    */
    let count: usize = list1
        .iter()
        .zip(list2.iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum();

    println!("Part 1 solution: {count}");

    let list2populations: HashMap<usize, usize> =
        list2.iter().copied().count_dupes().collect();

    let answer2: usize = list1
        .iter()
        .map(|lhs| lhs * list2populations.get(lhs).copied().unwrap_or(0))
        .sum();

    println!("Part 2 solution: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dupe_counter_counting() {
        let test_data = [1, 2, 3, 3, 3, 4, 5, 5];
        let counted: Vec<(i32, usize)> =
            test_data.into_iter().count_dupes().collect();
        assert_eq!(&counted, &[(1, 1), (2, 1), (3, 3), (4, 1), (5, 2)]);
    }
}
