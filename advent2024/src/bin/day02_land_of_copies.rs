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
        lists_of_levels.iter().filter(|x| is_safe_report(x)).count()
    );

    println!(
        "Part 2 solution: {}",
        lists_of_levels.iter().filter(|x| is_almost_safe_report(x)).count()
    );
}

fn is_safe_report(levels: &[i32]) -> bool {
    (levels.windows(2).all(|ab| {
        let a = ab[0];
        let b = ab[1];
        a > b
    }) | levels.windows(2).all(|ab| {
        let a = ab[0];
        let b = ab[1];
        a < b
    })) & levels.windows(2).all(|ab| {
        let a = ab[0];
        let b = ab[1];
        (a - b).abs() >= 1 && (a - b).abs() <= 3
    })
}

fn is_almost_safe_report(levels: &[i32]) -> bool {
    if is_safe_report(levels) {
        return true;
    }
    // for every index (i) in the list...
    for i in 0..levels.len() {
        // make a copy of the list
        let mut levels_copy = levels.to_vec();
        // delete element i of the list
        levels_copy.remove(i);
        // if is_safe_report() then return true
        if is_safe_report(&levels_copy) {
            return true;
        }
    }
    false
}
