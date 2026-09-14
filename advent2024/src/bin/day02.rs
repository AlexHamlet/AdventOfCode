use tal::{ConsecutiveOverlappingPairs, SkipNth};

fn main() {
    let mut lists_of_levels: Vec<Vec<_>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let levels: Vec<i32> =
            line.split(" ").map(|x| x.parse().unwrap()).collect();
        lists_of_levels.push(levels);
    }

    println!(
        "Part 1 solution: {}",
        lists_of_levels
            .iter()
            .filter(|x| is_safe_report(x.iter().copied()))
            .count()
    );

    println!(
        "Part 2 solution: {}",
        lists_of_levels
            .iter()
            .filter(|x| is_almost_safe_report(x))
            .count()
    );
}

fn is_safe_report(levels: impl Iterator<Item = i32> + Clone) -> bool {
    let mut levels = levels.consecutive_overlapping_pairs();
    let (first, second) = levels.clone().next().unwrap();
    let slope = (second - first).signum();
    levels.all(|(a, b)| {
        (b - a).signum() == slope && (a - b).abs() >= 1 && (a - b).abs() <= 3
    })
}

fn is_almost_safe_report(levels: &[i32]) -> bool {
    if is_safe_report(levels.iter().copied()) {
        return true;
    }
    for i in 0..levels.len() {
        if is_safe_report(levels.iter().copied().skip_nth(i)) {
            return true;
        }
    }
    false
}
