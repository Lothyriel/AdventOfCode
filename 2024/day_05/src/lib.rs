use std::collections::HashSet;

pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, updates) = input.split_once("\n\n").expect("Two parts");

    let rules = rules
        .lines()
        .filter_map(|line| line.split_once('|'))
        .map(|(p1, p2)| {
            let p1 = p1.parse().expect("Number");
            let p2 = p2.parse().expect("Number");

            (p1, p2)
        })
        .collect();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().expect("Number"))
                .collect()
        })
        .collect();

    (rules, updates)
}
