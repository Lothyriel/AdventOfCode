use std::collections::{HashMap, HashSet};

pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('-');

            let mut p = || parts.next().expect("Part");

            (p(), p())
        })
        .collect()
}

fn build_graph<'a>(connections: &[(&'a str, &'a str)]) -> HashMap<&'a str, HashSet<&'a str>> {
    connections
        .iter()
        .fold(HashMap::new(), |mut acc, (from, to)| {
            acc.entry(from).or_default().insert(to);
            acc.entry(to).or_default().insert(from);
            acc
        })
}
