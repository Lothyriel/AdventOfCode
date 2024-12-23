use std::collections::{HashMap, HashSet};

use crate::{build_graph, parse};

pub fn lan_party(input: &str) -> String {
    let connections = parse(input);

    let graph = build_graph(&connections);

    let mut largest = get_largest_cycle(&graph);
    largest.sort();
    largest.join(",")
}

fn get_largest_cycle<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> Vec<&'a str> {
    let mut largest = vec![];

    for &node in graph.keys() {
        let current = get_cycle(node, graph);

        if current.len() > largest.len() {
            largest = current;
        }
    }

    largest
}

fn get_cycle<'a>(node: &'a str, graph: &HashMap<&'a str, HashSet<&'a str>>) -> Vec<&'a str> {
    let mut current = vec![node];

    for neighbor in &graph[node] {
        if current.iter().all(|n| graph[neighbor].contains(n)) {
            current.push(neighbor);
        }
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

        let result = lan_party(input);
        assert_eq!("co,de,ka,ta", result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = lan_party(input);
        assert_eq!("am,au,be,cm,fo,ha,hh,im,nt,os,qz,rr,so", result);
    }
}
